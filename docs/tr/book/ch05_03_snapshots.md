# Bölüm 5.3: Snapshot ve Budama

Snapshot ancak deterministik replay için yeterli konsensüs state'ini taşıyorsa değerlidir. Pruning ise yalnız güvenli recovery kanıtlandığında açılmalıdır. Bu nedenle Budlum snapshot ve budama özelliklerini Mainnet varsayılanı değil, aşamalı hardening işi olarak ele alır.

## 1. Güncel Runtime Yolu

`StateSnapshot` bugün runtime'a bağlı legacy formattır. Chain kimliği, yükseklik, blok hash'i, bakiyeler, nonce değerleri, validatörler, finalized checkpoint bilgisi ve bütünlük hash'i taşır. P2P snapshot uygulaması farklı `chain_id` değerini ve yerel finality değerinden eski snapshot'ı reddeder.

`PruningManager` yalnız `features.pruning = true` olduğunda oluşturulur. Mainnet v1 bu feature flag'i reddeder; bu yüzden Mainnet yaklaşımı archive-first'tür.

## 2. StateSnapshotV2

`StateSnapshotV2` sonraki format olarak uygulanmış ve test edilmiştir. Şunları ekler:

- `schema_version`, `genesis_hash` ve oluşturulma zamanı,
- validatörler, unbonding kuyruğu ve finality sertifikaları,
- epoch indeksi, epoch zamanı, base fee ve block reward,
- bridge, message, settlement ve global-header özet kökleri.

Snapshot dosyaları yükseklik değerine göre sayısal sıralanır. En yeni JSON parse edilemiyorsa veya bütünlük hash'i geçersizse `.json.corrupted` uzantısıyla karantinaya alınır.

## 3. Eksik Production İşleri

V2 save/load yardımcıları vardır; ancak canlı node henüz V2'yi canonical restore ve fast-sync formatı olarak kullanmaz. Kimliği doğrulanmış snapshot dağıtımı, chunk-session bağlama, restore tatbikatları, replay eşdeğerliği testleri, archive-node politikası ve operasyon runbook'ları tamamlanmalıdır.

## Özet

Snapshot aşamalı bir recovery alt sistemidir. V2 restore yolu uçtan uca kanıtlanmadan Mainnet v1 için pruning kapalı kalmalıdır.
