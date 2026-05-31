# Bölüm 7: Ağ Ayrımı ve Katı Config V2

Budlum Mainnet, Testnet ve Devnet profillerini ayırır. Amaç chain kimliği, peer'lar, genesis dosyaları, anahtarlar ve operasyon beklentilerinin yanlışlıkla karışmasını engellemektir.

## 1. Yerleşik Profiller

| Ağ | Chain ID | Varsayılan P2P Portu | Kullanım |
| --- | ---: | ---: | --- |
| Mainnet | `1` | `4001` | production hedefi |
| Testnet | `42` | `5001` | kontrollü test ağı |
| Devnet | `1337` | `6001` | yerel geliştirme |

Yerleşik bootnode ve DNS seed listeleri bilinçli olarak boştur. Release operatörleri placeholder adres kullanmak yerine imzalı deployment konfigürasyonu hazırlamalıdır.

## 2. Katı Config V2

Repo `config/devnet.toml`, `config/testnet.toml` ve `config/mainnet.toml` örneklerini içerir. V2; `network`, `node`, `storage`, `p2p`, `rpc`, `metrics`, `validator` ve `features` bölümlerini tipli olarak parse eder. Bilinmeyen alanlar reddedilir. Önce dosya, sonra environment override uygulanır; katı runtime doğrulaması config dosyası olmadan başlatmalarda da çalışır.

Desteklenen roller `validator`, `sentry`, `seed`, `rpc` ve `archive` değerleridir. Validator, sentry ve seed rolleri public RPC başlangıcını kapatır.

## 3. Fail-Closed Kuralları

- Ayarlanmış chain ID seçili ağ profiliyle eşleşmelidir.
- Mevcut veritabanı açıldığında kayıtlı genesis kimliği seçili zincirle eşleşmelidir.
- Mainnet açık genesis ve seed/bootnode ayarı ister, mDNS kullanımını reddeder.
- Mainnet v1 governance, BudZKVM contract ve pruning özelliklerini reddeder.
- Mainnet validator başlangıcı PKCS#11 ayarlarını ister ve konsensüs signer adapter'ı henüz bağlı olmadığı için bilinçli olarak durur.

Bu kurallar koruyucu bariyerlerdir; Mainnet lansmanının hazır olduğu anlamına gelmez. Ayrıntılar için [Bölüm 12](ch12_production_hardening.md).
