# Chapter 4.3: Network Protocol and Messaging

This chapter explains the language Budlum machines use to talk to one another: `NetworkMessage`, serialization, sync requests, and network limits.

## 1. Data Structures: Shared Language

# Protocol Messages and Serialization

Budlum nodes use a custom `NetworkMessage` protocol for peer-to-peer communication and data sharing.

## What Does `NetworkMessage` Include?

1.  **Handshake / HandshakeAck:** new peers verify version and `chain_id`. `validator_set_hash` and `supported_schemes` are exchanged and logged, but enforcement policy is still pending.
2.  **Block:** broadcasts a newly produced block.
3.  **Transaction:** broadcasts new transactions.
4.  **Prevote / Precommit:** BLS finality votes produced by validators. Votes are signed with the validator's BLS secret key (`BlsKeypair` from `ValidatorKeys`) using `sign_bls()`. Individual signatures are verified via `verify_bls_sig()` before entering the `FinalityAggregator`.
5.  **FinalityCert:** threshold-signed BLS aggregate proof that a checkpoint was finalized. The aggregate signature is verified against the validator set's aggregated G2 public keys via pairing check.
6.  **GetQcBlob / QcBlobResponse:** shares Dilithium-signed blob packages for optimistic QC verification.
7.  **QcFaultProof:** broadcasts proof bytes for invalid PQ attestations.
8.  **SlashingEvidence:** gossips validator double-sign evidence so producers other than the detecting node can include it.
9.  **NewTip and sync messages:** used for chain synchronization, including requests such as `GetBlocksByHeight`.

The full list should be checked in `src/network/protocol.rs`.

## BLS Finality Protocol (v0.3-dev)

The finality protocol operates in three phases at each checkpoint height (every 10 blocks):

1. **Prevote Phase:** When a block is produced at a checkpoint height, the producing node automatically starts the prevote phase. Validators sign prevote messages with their BLS secret key (`sign_bls`) and broadcast them via GossipSub. The `FinalityAggregator` tracks votes and checks for 2/3 stake quorum.

2. **Precommit Phase:** Once the prevote quorum is reached, validators automatically sign and broadcast BLS precommit messages. The periodic voting loop in `Node` checks `get_aggregator_state()` and triggers auto-precommit when prevote quorum is detected.

3. **Certificate Production:** When the precommit quorum (2/3 stake) is reached, the aggregator produces a `FinalityCert` containing the aggregated BLS G1 signature, a signer bitmap, and the validator set hash. The cert is gossiped network-wide and verified via BLS pairing (`e(sig, -G2_gen) + e(H(msg), agg_pk) == 0`).

The gossip path is: `GossipSub` → `Node` → `ChainHandle::handle_prevote/handle_precommit` → `ChainActor` → `Blockchain::finality_aggregator`.

## Publishing with Gossipsub

Budlum Core uses **Gossipsub** for broad announcements such as blocks and transactions. Gossip is fast, but it is not ideal for large historical transfers.

## Request-Response Synchronization

Budlum includes one-to-one **Request-Response** sync support for large historical transfers.

- **Protocol ID:** `/budlum/sync/1.0.0`
- **SyncCodec:** length-prefixed serialization over streams.
- **Actor integration:** `Node` forwards incoming requests to `ChainActor`, serving blocks and headers without global locking.
- **Handshake-triggered sync:** if a peer's handshake reports a higher best height, the node automatically requests headers and reports the real sync state through `bud_syncing`.

## QC Messages

- `GetQcBlob { epoch, checkpoint_height }`: asks peers for the PQ sidecar required by a checkpoint.
- `QcBlobResponse { epoch, checkpoint_height, checkpoint_hash, blob_data, found }`: carries the serialized blob. The receiver parses it, checks metadata, verifies the Merkle root and Dilithium signatures, persists it as `QC_BLOB:{height}`, and retries pending finality certificates for that checkpoint.
- `QcFaultProof { proof_data }`: carries a serialized `QcFaultProof`. The receiver verifies it against the stored blob and validator snapshot before applying the verdict.

## Slashing Evidence Gossip

`NetworkMessage::SlashingEvidence` carries verified PoS equivocation evidence across the mesh. Receiving nodes submit the evidence to `ChainActor`, rebroadcast valid evidence, and producers drain pending evidence into later blocks for deterministic slashing execution.

## Snapshot Sync (V2)

Nodes request state snapshots via `GetStateSnapshot { height }`. Responding nodes generate `StateSnapshotV2` data (with full consensus metadata) and stream it in 512KB chunks via `SnapshotChunk` messages. Each chunk carries a `session_id` to prevent cross-session chunk mixing. Completed snapshots are reassembled and applied via `apply_state_snapshot()` with V2 metadata restore.

## Serialization

Budlum uses a hybrid serialization model:

- **Protobuf:** high-performance network messages and core structures.
- **Serde JSON:** readable high-level configuration and diagnostic data.
- **Bincode:** deterministic byte-for-byte encoding for embedded slashing evidence and similar structures.

## Limits and Security

Network input is untrusted. Message size limits prevent memory exhaustion attacks, and oversized blocks or messages are rejected automatically.

Current protocol constants are 10 MiB for a network message, 1 MiB for a block, 100 KiB for a transaction, 500 blocks for chain-sync batches, and 256 blocks for snapshot-sync batches.
