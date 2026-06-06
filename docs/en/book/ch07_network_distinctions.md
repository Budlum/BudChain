# Chapter 7: Network Distinctions and Strict Config V2

Budlum separates Mainnet, Testnet, and Devnet so chain identity, peers, genesis files, keys, and operational expectations cannot mix accidentally.

## 1. Built-In Profiles

| Network | Chain ID | Default P2P Port | mDNS | Persistent Identity | Durable Bans |
| --- | ---: | ---: | --- | --- | --- |
| Mainnet | `1` | `4001` | Disabled | Required | Enabled |
| Testnet | `42` | `5001` | Disabled | Required | Enabled |
| Devnet | `1337` | `6001` | Enabled | Optional | Disabled |

Built-in bootnode and DNS-seed arrays are intentionally empty. Release operators must populate signed deployment configuration rather than rely on placeholder addresses.

## 2. P2P Hardening (v0.3-dev)

### Persistent Identity

Nodes load their P2P identity key from `p2p_identity_file` via `load_or_generate_identity_key()`. On first run, a new Ed25519 keypair is generated and saved to the file path. On subsequent runs, the existing key is loaded, preserving the node's `PeerId` across restarts. Without an identity file, the node generates an ephemeral key suitable only for Devnet.

The identity file is created with `0o600` permissions on Unix. Failed reads or corrupt data trigger a fresh key generation with a logged warning.

### Durable Peer Bans

When `persist_banned_peers` is enabled (Mainnet, Testnet), currently banned `PeerId`s are serialized as JSON to `banned_peer_db` every 5 minutes. On node startup, the file is loaded and all listed peers are re-banned for the standard ban duration. Devnet keeps bans in-memory only.

### mDNS Policy

The `mdns_enabled` flag from `Network::security_config()` is honored at runtime. Mainnet and Testnet disable mDNS; Devnet permits it. When disabled, mDNS discovery events are silently skipped in the event loop, preventing local-network peer discovery.

### DNS Seed Resolution

`dns_seeds` from configuration are resolved via `resolve_dns_seeds()` at startup. DNS hostnames are converted to `/ip4/` or `/ip6/` multiaddrs and dialed. Resolution failures are logged but do not prevent startup.

## 3. Strict Config V2

The repository includes `config/devnet.toml`, `config/testnet.toml`, and `config/mainnet.toml`. V2 uses typed sections: `network`, `node`, `storage`, `p2p`, `rpc`, `metrics`, `validator`, and `features`. Unknown fields are rejected. File values load first, environment overrides apply second, and strict runtime validation runs even without a config file.

Supported roles are `validator`, `sentry`, `seed`, `rpc`, and `archive`. Validator, sentry, and seed roles disable public RPC startup; operator RPC remains available for admin management.

## 4. Fail-Closed Rules

- A configured chain ID must match the selected network profile.
- Stored genesis identity must match the selected chain when an existing database is opened.
- Mainnet requires explicit genesis and seed/bootnode configuration and rejects mDNS.
- Mainnet v1 rejects governance, BudZKVM contracts, and pruning.
- Mainnet validator startup requires PKCS#11 configuration.

These rules are guardrails, not a declaration that Mainnet launch is ready. See [Chapter 12](ch12_production_hardening.md).
