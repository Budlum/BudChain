# Bölüm 12: Production Hardening Durumu

Bu bölüm reponun güncel operasyonel gerçeklik tablosudur. Budlum Core kontrollü public-devnet adayıdır. Denetlenmiş Mainnet yazılımı değildir ve gerçek ekonomik değer taşımamalıdır.

## 1. Uygulanan Korumalar

| Alan | Güncel davranış |
| --- | --- |
| Konfigürasyon | Katı Config V2 bilinmeyen alanları, profil/chain-ID uyumsuzluğunu, güvensiz Mainnet feature flag'lerini, eksik Mainnet genesis'i, boş Mainnet seed ayarını ve Mainnet mDNS kullanımını reddeder. |
| Genesis | `genesis build` private key materyalini yazdırmaz. Otomatik allocation anahtarı yalnız devnet için ve açık output dosyasıyla üretilebilir. Devnet dışı genesis açık validatör listesi ister. |
| Başlangıç | Storage açılış hataları başlangıcı durdurur. Mevcut DB ayarlanan genesis kimliğine karşı kontrol edilir. Özel genesis dosyası parse edilmeli ve seçili chain ID ile eşleşmelidir. |
| State commitment | `ConsensusStateV2` hesapları, validatörleri, unbonding kuyruğunu, ekonomiyi, bridge, message, settlement ve global-header özet state'ini bağlar. |
| Kalıcılık | Canonical değişiklikler `IN_PROGRESS_HEIGHT` recovery marker'ı ve atomik Sled batch içeren `DurableCommitBatch` kullanır. |
| Snapshot aşaması | Snapshot dosyaları sayısal sıralanır; bozuk en yeni dosya karantinaya alınır. `StateSnapshotV2` genişletilmiş konsensüs metadata'sı taşır. |
| RPC tabanı | HTTP middleware API-key auth, CORS allowlist, allowed-IP filtresi ve global dakikalık istek penceresi sağlar. |
| CI | GitHub Actions Rust `1.94.0` sürümünü pinler; format, `cargo check`, warning'leri reddeden Clippy, workspace testleri ve `--release --locked` build çalıştırır. |

## 2. Aşamalı veya Kısmi İşler

| Alan | Sınır |
| --- | --- |
| Finality | Agregasyon ve sertifika doğrulama mantığı testlidir; canlı imzalı vote üretimi ve ağ agregasyonu uçtan uca bağlı değildir. |
| P2P | Version ve chain ID zorlanır. Validator-set hash ve scheme policy, kalıcı kimlik, profil kontrollü mDNS, DNS seed ve kalıcı ban runtime bağlantıları eksiktir. |
| RPC | Config public/operator listener ve trusted proxy alanlarını parse eder; runtime tek listener başlatır. Header tabanlı IP değeri trusted-proxy kısıtına bağlı değildir. |
| Metrics | Prometheus tanımları ve endpoint vardır; canlı collector'ların çoğu ve listener binding politikası eksiktir. |
| Snapshot V2 | Format ve yardımcılar vardır; V2 henüz canonical restore veya fast-sync yolu değildir. |
| Storage | Durable block commit vardır; tam kalıcı ConsensusStateV2 zarfı, release migration'ları ve restore tatbikatları eksiktir. |

## 3. Açık Mainnet Engelleri

1. PKCS#11 konsensüs signer adapter'ını uygula ve denetle. Mainnet validator başlangıcı bu adapter gelene kadar bilinçli olarak reddedilir.
2. İmzalı prevote/precommit üretimini, canlı agregasyonu, sertifika yayılımını ve saldırgan çok node'lu finality testlerini tamamla.
3. Public ve operator RPC sunucularını ayır; trusted proxy zorlaması, health endpoint, connection/body limitleri ve istemci bazlı quota ekle.
4. Kalıcı P2P kimliği, discovery politikası, DNS seed ve kalıcı peer ban bağlantılarını tamamla.
5. Snapshot V2 restore, kimliği doğrulanmış dağıtım, chunk-session bağlama, replay eşdeğerliği, backup restore tatbikatı ve archive politikasını tamamla.
6. Governance, BudZKVM contract ve pruning özelliklerini ayrı incelemeler tamamlanana kadar Mainnet v1 için kapalı tut.
7. Deployment paketleri, release ceremony kayıtları, dashboard'lar, incident runbook'ları, fault injection, fuzzing sonuçları ve dış güvenlik denetimi üret.

## 4. Release Kapıları

Her release adayı şu komutları geçmelidir:

```bash
nix develop --command cargo fmt --all -- --check
nix develop --command cargo clippy --workspace --all-targets --all-features -- -D warnings
nix develop --command cargo test --workspace
nix develop --command cargo build --release --locked
git diff --check
```

Kritik adapter'lar tamamlanmadığı sürece Mainnet profili bilinçli olarak fail-closed davranır.
