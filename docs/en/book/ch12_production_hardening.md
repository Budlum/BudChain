# Chapter 12: Production Hardening Status

This chapter is the operational truth table for the current repository. Budlum Core is a controlled public-devnet candidate. It is not audited Mainnet software and must not carry real economic value.

## 1. Implemented Protections

| Area | Current behavior |
| --- | --- |
| Configuration | Strict Config V2 rejects unknown fields, profile/chain-ID mismatches, unsafe Mainnet feature flags, missing Mainnet genesis, empty Mainnet seed configuration, and Mainnet mDNS. |
| Genesis | `genesis build` never prints private key material. Automatic allocation-key generation is devnet-only and requires an explicit output file. Non-devnet genesis requires explicit validators. |
| Startup | Storage open failures stop startup. Existing databases are checked against configured genesis identity. Custom genesis files must parse and match the selected chain ID. |
| State commitment | `ConsensusStateV2` commits accounts, validators, unbonding, economics, bridge, message, settlement, and global-header summary state. |
| Persistence | Canonical changes use `DurableCommitBatch` with an `IN_PROGRESS_HEIGHT` recovery marker and atomic Sled write batch. |
| Snapshot staging | Snapshot files are numerically ordered; corrupt latest files are quarantined. `StateSnapshotV2` captures expanded consensus metadata. |
| RPC baseline | HTTP middleware supports API-key auth, CORS allowlists, allowed-IP filtering, and a global per-minute request window. |
| CI | GitHub Actions pins Rust `1.94.0`, checks formatting, runs `cargo check`, denies Clippy warnings, executes workspace tests, and builds `--release --locked`. |
| PKCS#11 | `ConsensusSigner` trait + `Pkcs11Signer` adapter (via `cryptoki`) + `KeyPairSigner` local fallback. `ConsensusEngine` trait exposes `fn signer()`. Block signing uses HSM when configured, with local file fallback. Mainnet startup guard passes. |

## 2. Staged or Partial Work

| Area | Boundary |
| --- | --- |
| Finality | Prevote/Precommit structs, `FinalityAggregator`, certificate production and verification are all implemented and tested. Gossip messages are forwarded to the `ChainActor`, which dispatches them to the `Blockchain`'s aggregator. `ProduceBlock` at checkpoint heights starts the prevote phase. **Remaining:** BLS-signed vote production from validators, quorum-triggered certificate gossip broadcast, and adversarial multi-node finality tests. |
| P2P | Version and chain ID are enforced. Validator-set hash and supported-scheme policy, persistent identities, profile-controlled mDNS, DNS seeds, and durable bans still need runtime wiring. |
| RPC | Config parses public/operator listeners and trusted proxies, but runtime starts one listener. Header-derived client IPs are not trusted-proxy constrained. |
| Metrics | Prometheus descriptors and endpoint exist, but most live collectors and listener binding policy are incomplete. |
| Snapshot V2 | Format and helpers exist, but V2 is not yet the canonical restore or fast-sync path. |
| Storage | Durable block commit exists, but a complete persisted ConsensusStateV2 envelope, released migrations, and restore drills remain. |

## 3. Explicit Mainnet Blockers

1.  ~~Implement and audit the PKCS#11 consensus signer adapter.~~ **DONE (v0.2-dev):** `Pkcs11Signer` with `cryptoki` HSM backend, `ConsensusSigner` trait, `KeyPairSigner` local fallback, and `ConsensusEngine::signer()` wired into PoS/PoA block production.
2.  Complete BLS-signed prevote/precommit vote **production** from validators, live certificate gossip broadcast, and adversarial multi-node finality tests. (Aggregation, dispatch, and cert verification are wired and tested.)
3.  Separate public and operator RPC servers, enforce trusted proxies, add health endpoints, and define connection/body limits and per-client quotas.
4.  Wire persistent P2P identity, discovery policy, DNS seeds, and durable peer bans.
5.  Finish Snapshot V2 restore, authenticated distribution, chunk-session binding, replay equivalence, backup restore drills, and archive policy.
6.  Keep governance, BudZKVM contracts, and pruning disabled for Mainnet v1 until separately reviewed.
7.  Produce deployment packaging, release ceremony records, observability dashboards, incident runbooks, fault injection results, fuzzing results, and an external security audit.

## 4. Release Gates

Every release candidate should run:

```bash
nix develop --command cargo fmt --all -- --check
nix develop --command cargo clippy --workspace --all-targets --all-features -- -D warnings
nix develop --command cargo test --workspace
nix develop --command cargo build --release --locked
git diff --check
```

The Mainnet profile is deliberately fail-closed while critical adapters remain incomplete.
