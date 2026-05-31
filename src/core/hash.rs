use sha2::{Digest, Sha256};
pub fn calculate_hash(data: &[u8]) -> String {
    hex::encode(calculate_hash_bytes(data))
}
pub fn calculate_hash_bytes(data: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}
pub fn hash_fields(fields: &[&[u8]]) -> String {
    hex::encode(hash_fields_bytes(fields))
}
pub fn hash_fields_bytes(fields: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    for field in fields {
        hasher.update((field.len() as u64).to_le_bytes());
        hasher.update(field);
    }
    hasher.finalize().into()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_hash() {
        let hash1 = calculate_hash(b"hello");
        let hash2 = calculate_hash(b"hello");
        let hash3 = calculate_hash(b"world");
        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
        assert_eq!(hash1.len(), 64);
    }
    #[test]
    fn test_hash_fields_bytes_prevents_collision() {
        let fields1 = vec![b"a".as_slice(), b"bc".as_slice()];
        let fields2 = vec![b"ab".as_slice(), b"c".as_slice()];
        let hash1 = hash_fields_bytes(&fields1);
        let hash2 = hash_fields_bytes(&fields2);
        assert_ne!(hash1, hash2);
    }
}
