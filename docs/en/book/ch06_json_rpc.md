# Chapter 6: JSON-RPC 2.0 API

The JSON-RPC API is Budlum's integration layer for wallets, dashboards, scripts, and external services.

## 1. Running

The RPC server is configured through TOML and exposes Budlum-specific methods with the `bud_` prefix.

### Configuration File

Typical configuration includes bind addresses for public and operator listeners, optional auth settings, CORS origins, allowed IPs, trusted proxies, and per-IP rate limiting.

### Public vs Operator Listeners

Budlum starts **two separate RPC servers**:

| Listener | Bind | Auth | Rate Limit | Body Limit | Max Conn | Purpose |
| --- | --- | --- | --- | --- | --- | --- |
| Public | Configurable (e.g. `0.0.0.0:8545`) | API key required (mainnet/testnet) | Per-IP, configurable | 10 MB | 500 | External wallets, dApps |
| Operator | `127.0.0.1:8546` | None (localhost) | None | 50 MB | 10 | Admin, health checks |

Trusted proxies: Only requests originating from configured proxy IPs may use `X-Forwarded-For` for client IP identification. Unproxied requests use `X-Real-IP`.

## 2. Health and Status Endpoints

| Method | Description |
| --- | --- |
| `bud_health` | Returns `status` ("healthy" or "syncing"), `blockHeight`, `peerCount`, and `syncing` flag. |
| `bud_nodeInfo` | Returns `chainId`, `blockHeight`, `validatorSetHash`, `syncState`, `peerCount`, `peerId`, and `rpcMode` ("public" or "operator"). |

## 3. Observability: Prometheus Metrics

Budlum exposes a Prometheus text-format endpoint at `:{metrics_port}/metrics`. Live collectors are wired for chain height, finalized height, finality lag, blocks produced, transactions processed, reorgs, mempool size/evictions/cleanups, and P2P messages received/peer count.

## 4. Supported Methods (`bud_` Prefix)

Common methods include:

- `bud_blockNumber`
- `bud_getBalance`
- `bud_sendRawTransaction`
- `bud_getTransaction`
- `bud_getBlockByNumber`
- `bud_txPrecheck`
- `bud_submitVerifiedDomainCommitment`
- `bud_registerBridgeAsset`
- `bud_lockBridgeTransfer`
- `bud_mintBridgeTransfer`
- `bud_burnBridgeTransferWithEvent`
- `bud_unlockBridgeTransferVerified`
- `bud_health`
- `bud_nodeInfo`

Settlement and bridge methods:

| Method | Purpose |
| --- | --- |
| `bud_getSettlementInfo` | Returns pending global settlement roots and domain commitment count. |
| `bud_getGlobalHeader` | Returns a sealed global header by height. |
| `bud_getDomainCommitments` | Lists domain commitments currently known to settlement. |
| `bud_getConsensusDomains` | Lists registered consensus domains. |
| `bud_registerConsensusDomain` | Registers a domain with operator, bond, adapter, and validator-set metadata. |
| `bud_submitDomainCommitment` | Disabled. Raw commitment submission is rejected; use verified submission. |
| `bud_submitVerifiedDomainCommitment` | Submits a commitment plus finality proof. The proof hash, adapter, validator-set anchor, and finality status are checked before acceptance. |
| `bud_registerBridgeAsset` | Registers an asset for a bridge-enabled source domain. |
| `bud_lockBridgeTransfer` | Creates a source-domain bridge lock. Source and target domains must both be registered, active, bridge-enabled, and distinct. |
| `bud_mintBridgeTransfer` | Mints from a verified source-domain `BridgeLocked` event proof. |
| `bud_burnBridgeTransfer` | Disabled raw burn path. Use `bud_burnBridgeTransferWithEvent`. |
| `bud_burnBridgeTransferWithEvent` | Burns on the target side and returns a `BridgeBurned` event that must be committed by the target domain. |
| `bud_unlockBridgeTransfer` | Disabled raw unlock path. Use `bud_unlockBridgeTransferVerified`. |
| `bud_unlockBridgeTransferVerified` | Unlocks source funds only after verifying a committed target-domain `BridgeBurned` event Merkle proof. |
| `bud_sealGlobalHeader` | Seals the current deterministic settlement roots into a global header. |
| `bud_health` | Health status, block height, peer count, sync state. |
| `bud_nodeInfo` | Full node identity: chain ID, peer ID, validator set hash, RPC mode. |

## 5. Example Usage

### Query Block Count

Use JSON-RPC over HTTP to call `bud_blockNumber`.

### Health Check

```bash
curl -X POST -H "Content-Type: application/json" \
  --data '{"jsonrpc":"2.0","method":"bud_health","params":[],"id":1}' \
  http://localhost:8545
```

### BudZKVM ContractCall Precheck

`bud_txPrecheck` validates transaction shape and BudZKVM bytecode alignment before the user pays to propagate or include the transaction.

## 6. Architecture and Security

1. **Transaction validation:** `bud_sendRawTransaction` checks size and cryptographic signature before gossip.
2. **Per-IP rate limiting:** Each client IP gets its own 60-second sliding window token bucket. Different IPs do not share rate limits.
3. **Trusted proxy enforcement:** `X-Forwarded-For` is only honored from IPs in the `trusted_proxies` configuration list. Otherwise, `X-Real-IP` or a direct remote address is used.
4. **Config-based auth and rate limiting:** TOML fields `auth_required`, `api_key_env`, `allowed_ips`, `cors_origins`, `trusted_proxies`, and `rate_limit_per_minute` are enforced by the RPC HTTP middleware. Auth accepts `x-api-key` or `Authorization: Bearer ...`.
5. **Body and connection limits:** Public server limits: 10MB body, 500 connections. Operator server limits: 50MB body, 10 connections.
6. **ContractCall shape checks:** precheck and mempool validation reject empty or misaligned BudZKVM bytecode.
7. **Verified settlement only:** raw domain commitments, bridge burns, and bridge unlocks are rejected by RPC. Settlement-changing bridge return paths require committed domain events and Merkle proofs.

## 7. How Realistic Is `bud_txPrecheck`?

`bud_txPrecheck` is a fast early warning system. It does not replace full block execution, but it helps wallets and operators catch malformed transactions before broadcasting them.
