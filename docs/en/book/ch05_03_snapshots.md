# Chapter 5.3: Snapshots and Pruning

Snapshots are useful only when they preserve enough consensus state to replay deterministically. Pruning is useful only when operators can recover safely. Budlum therefore treats both as staged hardening work, not as a Mainnet default.

## 1. Legacy Runtime Path

`StateSnapshot` is the currently connected runtime format. It stores chain identity, height, block hash, balances, nonces, validators, finalized checkpoint metadata, and an integrity hash. P2P snapshot application rejects mismatched chain IDs and snapshots older than local finality.

`PruningManager` is created only when `features.pruning = true`. Mainnet v1 rejects that feature flag, so the Mainnet posture is archive-first.

## 2. StateSnapshotV2

`StateSnapshotV2` is implemented and tested as the next format. It adds:

-   `schema_version`, `genesis_hash`, and creation time,
-   validators, unbonding queue, and finality certificates,
-   epoch index, epoch timing, base fee, and block reward,
-   bridge, message, settlement, and global-header summary roots.

Snapshot files are ordered by numeric height. If the newest JSON file cannot be parsed or fails its integrity hash, it is renamed with `.json.corrupted` so operators can investigate it.

## 3. What Is Still Missing?

V2 save/load helpers exist, but the live node does not yet use V2 as its canonical restore and fast-sync format. Production work still includes authenticated snapshot distribution, chunk-session binding, restore drills, replay-equivalence tests, archive-node policy, and operator runbooks.

## Summary

Snapshots are a staged recovery subsystem. Mainnet v1 must keep pruning disabled until the V2 restore path and operational procedures are proven end to end.
