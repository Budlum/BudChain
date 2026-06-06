# Chapter 5.3: Snapshots and Pruning

Snapshots are useful only when they preserve enough consensus state to replay deterministically. Pruning is useful only when operators can recover safely. Budlum therefore treats both as staged hardening work, not as a Mainnet default.

## 1. Runtime Path (V2 Canonical)

`StateSnapshotV2` is the canonical runtime format as of v0.3-dev. It stores:

- **Chain identity:** `schema_version` (2), height, block hash, genesis hash, chain ID.
- **Account state:** balances, nonces, validators (with BLS public key, PoP signature, PQ public key), unbonding queue.
- **Consensus metadata:** epoch index, last epoch time, base fee, block reward.
- **Cross-domain roots:** bridge root, message root, settlement root, global header summary.
- **Finality data:** finalized height, finalized hash, verified finality certificates.
- **Integrity:** SHA3-256 hash over all fields.

`PruningManager` is created only when `features.pruning = true`. Mainnet v1 rejects that feature flag, so the Mainnet posture is archive-first.

## 2. V2 Canonical Restore

The live node uses V2 as its primary restore format:

- **Startup:** `Blockchain::new_with_genesis()` tries `load_latest_snapshot_v2()` before falling back to V1.
- **AccountState:** `from_snapshot_v2()` preserves all consensus metadata (epoch, base fee, block reward, unbonding queue, cross-domain roots, finality certs), ensuring replay equivalence.
- **Runtime snapshots:** `validate_and_add_block()` produces V2 snapshots with `StateSnapshotV2::from_state()` and `save_snapshot_v2()`.
- **P2P:** `get_state_snapshot()` generates V2 data; `apply_state_snapshot()` detects `__v2__` prefix and routes to `apply_v2_snapshot()` for full metadata restore including finality certificates.
- **Chunk-session binding:** `SnapshotChunk` carries a `session_id` field. Receivers reject chunks from stale sessions to prevent cross-peer chunk mixing.

## 3. Snapshot File Management

Snapshot files are ordered by numeric height. If the newest JSON file cannot be parsed or fails its integrity hash, it is renamed with `.json.corrupted` so operators can investigate it.

## 4. What Is Still Missing?

Snapshots are a staged recovery subsystem. Remaining work includes archive-node policy, authenticated snapshot distribution (BLS-signature on snapshot data), backup restore drills with full chain replay verification, and operator runbooks.

## Summary

V2 snapshots now capture full consensus metadata, enabling deterministic state reconstruction. Mainnet v1 must keep pruning disabled until the V2 restore path and operational procedures are proven end to end.
