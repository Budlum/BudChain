# Chapter 7: Network Distinctions and Strict Config V2

Budlum separates Mainnet, Testnet, and Devnet so chain identity, peers, genesis files, keys, and operational expectations cannot mix accidentally.

## 1. Built-In Profiles

| Network | Chain ID | Default P2P Port | Default Role |
| --- | ---: | ---: | --- |
| Mainnet | `1` | `4001` | deployment-defined |
| Testnet | `42` | `5001` | deployment-defined |
| Devnet | `1337` | `6001` | local development |

Built-in bootnode and DNS-seed arrays are intentionally empty. Release operators must populate signed deployment configuration rather than rely on placeholder addresses.

## 2. Strict Config V2

The repository includes `config/devnet.toml`, `config/testnet.toml`, and `config/mainnet.toml`. V2 uses typed sections: `network`, `node`, `storage`, `p2p`, `rpc`, `metrics`, `validator`, and `features`. Unknown fields are rejected. File values load first, environment overrides apply second, and strict runtime validation runs even without a config file.

Supported roles are `validator`, `sentry`, `seed`, `rpc`, and `archive`. Validator, sentry, and seed roles disable public RPC startup.

## 3. Fail-Closed Rules

-   A configured chain ID must match the selected network profile.
-   Stored genesis identity must match the selected chain when an existing database is opened.
-   Mainnet requires explicit genesis and seed/bootnode configuration and rejects mDNS.
-   Mainnet v1 rejects governance, BudZKVM contracts, and pruning.
-   Mainnet validator startup requires PKCS#11 configuration and intentionally stops because the consensus signer adapter is not wired yet.

These rules are guardrails, not a declaration that Mainnet launch is ready. See [Chapter 12](ch12_production_hardening.md).
