#[cfg(test)]
mod hardening_tests {
    use crate::cli::commands::NodeConfig;
    use crate::core::account::AccountState;
    use crate::core::address::Address;
    use crate::core::metrics::Metrics;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_merkle_state_root_determinism() {
        let mut state1 = AccountState::new();
        let alice = Address::from_hex(&"01".repeat(32)).unwrap();
        let bob = Address::from_hex(&"02".repeat(32)).unwrap();

        state1.add_balance(&alice, 100);
        state1.add_balance(&bob, 200);

        let mut state2 = AccountState::new();
        state2.add_balance(&bob, 200);
        state2.add_balance(&alice, 100);

        let root1 = state1.calculate_state_root();
        let root2 = state2.calculate_state_root();

        assert_eq!(
            root1, root2,
            "Merkle root must be deterministic regardless of insertion order"
        );
        assert_ne!(root1, "0".repeat(64), "Root should not be empty");

        state1.add_balance(&alice, 1);
        assert_ne!(
            root1,
            state1.calculate_state_root(),
            "Root must change when balance changes"
        );
    }

    #[test]
    fn test_metrics_encoding_format() {
        let metrics = Metrics::new();
        metrics.chain_height.set(1234);
        metrics.peer_count.set(5);

        let encoded = metrics.encode();
        assert!(
            encoded.contains("budlum_chain_height 1234"),
            "Encoded metrics should contain height"
        );
        assert!(
            encoded.contains("budlum_peer_count 5"),
            "Encoded metrics should contain peer count"
        );
        assert!(
            encoded.contains("# HELP budlum_chain_height"),
            "Encoded metrics should contain HELP metadata"
        );
    }

    #[test]
    fn test_toml_config_merge() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("budlum.toml");
        let mut file = File::create(&config_path).unwrap();
        writeln!(
            file,
            r#"
            [storage]
            data_dir = "/tmp/custom_db"
            [rpc]
            public_listener = "127.0.0.1:9999"
            [metrics]
            listener = "0.0.0.0:7070"
        "#
        )
        .unwrap();

        let mut config = NodeConfig {
            config: Some(config_path.to_str().unwrap().to_string()),
            ..Default::default()
        };

        assert_ne!(config.rpc_port, 9999);

        config.load_with_file();

        assert_eq!(config.db_path, "/tmp/custom_db");
        assert_eq!(config.rpc_port, 9999);
        assert_eq!(config.metrics_port, 7070);
    }

    #[test]
    fn test_apply_snapshot_rejects_older_than_finalized() {
        use crate::chain::blockchain::Blockchain;
        use crate::consensus::pow::PoWEngine;
        use std::sync::Arc;

        let consensus = Arc::new(PoWEngine::new(0));
        let mut bc = Blockchain::new(consensus, None, 1337, None);
        bc.finalized_height = 10;

        let snapshot = crate::chain::snapshot::StateSnapshot::from_state(
            5,
            "hash".to_string(),
            1337,
            &bc.state,
            0,
            "finalhash".to_string(),
        );

        let result = bc.apply_state_snapshot(snapshot);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("older than current finalized"));
    }

    #[test]
    fn test_db_repair_index() {
        use crate::core::block::Block;
        use crate::storage::db::Storage;

        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test_repair.db");
        let storage = Storage::new(db_path.to_str().unwrap()).unwrap();

        // Create a block and commit it
        let mut block = Block::new(1, "prev_hash".to_string(), vec![]);
        block.hash = block.calculate_hash();
        storage.commit_block(&block, "state_root_1").unwrap();

        // Verify we can read it
        assert!(storage.get_block_by_height(1).unwrap().is_some());

        // Corrupt the height index by removing it
        let height_key = format!("HEIGHT:{}", 1);
        storage.db().remove(height_key.as_bytes()).unwrap();
        storage.db().flush().unwrap();

        // Verify reading by height returns None now
        assert!(storage.get_block_by_height(1).unwrap().is_none());

        // Repair the index
        storage.repair_index().unwrap();

        // Verify reading by height works again
        assert!(storage.get_block_by_height(1).unwrap().is_some());
        assert_eq!(
            storage.get_block_by_height(1).unwrap().unwrap().hash,
            block.hash
        );
    }
}
