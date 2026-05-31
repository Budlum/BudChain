# Chapter 6: JSON-RPC 2.0 API

The JSON-RPC API is Budlum's integration layer for wallets, dashboards, scripts, and external services.

## 1. Running

The RPC server is configured through TOML and exposes Budlum-specific methods with the `bud_` prefix.

### Configuration File

Typical configuration includes bind address, optional auth settings, CORS origins, allowed IPs, and a global request-rate window.

## 2. Observability: Prometheus Metrics

Budlum exposes a Prometheus text-format endpoint. Metric descriptors exist for chain, storage, settlement, P2P, and mempool behavior. Most live collectors are not wired yet, and the metrics server currently binds `0.0.0.0:{metrics_port}` independently of the parsed listener string.

## 3. Supported Methods (`bud_` Prefix)

Common methods include:

-   `bud_blockNumber`
-   `bud_getBalance`
-   `bud_sendRawTransaction`
-   `bud_getTransaction`
-   `bud_getBlockByNumber`
-   `bud_txPrecheck`
-   `bud_submitVerifiedDomainCommitment`
-   `bud_registerBridgeAsset`
-   `bud_lockBridgeTransfer`
-   `bud_mintBridgeTransfer`
-   `bud_burnBridgeTransferWithEvent`
-   `bud_unlockBridgeTransferVerified`

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

## 4. Example Usage

### Query Block Count

Use JSON-RPC over HTTP to call `bud_blockNumber`.

### Query Balance

Call `bud_getBalance` with an account public key or address.

### BudZKVM ContractCall Precheck

`bud_txPrecheck` validates transaction shape and BudZKVM bytecode alignment before the user pays to propagate or include the transaction.

## 5. Architecture and Security

1.  **Transaction validation:** `bud_sendRawTransaction` checks size and cryptographic signature before gossip.
2.  **Config-based auth and rate limiting:** TOML fields `auth_required`, `api_key_env`, `allowed_ips`, `cors_origins`, and `rate_limit_per_minute` are enforced by the RPC HTTP middleware. Auth accepts `x-api-key` or `Authorization: Bearer ...`.
3.  **ContractCall shape checks:** precheck and mempool validation reject empty or misaligned BudZKVM bytecode.
4.  **Verified settlement only:** raw domain commitments, bridge burns, and bridge unlocks are rejected by RPC. Settlement-changing bridge return paths require committed domain events and Merkle proofs.

## 6. Mainnet Boundaries

Config V2 parses `public_listener`, `operator_listener`, and `trusted_proxies`, but runtime currently starts one HTTP RPC listener. Allowed-IP matching trusts `x-forwarded-for` or `x-real-ip` directly, so it must not be exposed behind arbitrary proxies. Separate public/operator servers, trusted-proxy enforcement, per-client quotas, health endpoints, and explicit connection/body limits remain Mainnet work.

## 7. How Realistic Is `bud_txPrecheck`?

`bud_txPrecheck` is a fast early warning system. It does not replace full block execution, but it helps wallets and operators catch malformed transactions before broadcasting them.
