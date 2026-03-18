#[cfg(test)]
mod tests {
    use crate::crypto::*;

    #[test]
    fn sha1_empty() {
        let h = sha1_of(b"");
        assert_eq!(
            hex::encode(h),
            "da39a3ee5e6b4b0d3255bfef95601890afd80709"
        );
    }

    #[test]
    fn sha1_hello() {
        let h = sha1_of(b"hello");
        assert_eq!(
            hex::encode(h),
            "aaf4c61ddcc5e8a2dabede0f3b482cd9aea9434d"
        );
    }

    #[test]
    fn sha1_of_words_matches_raw() {
        let words: [u32; 2] = [0x01020304, 0x05060708];
        let h1 = sha1_of_words(&words);
        let mut raw = Vec::new();
        for &w in &words {
            raw.extend_from_slice(&w.to_le_bytes());
        }
        let h2 = sha1_of(&raw);
        assert_eq!(h1, h2);
    }

    #[test]
    fn crc32_empty() {
        assert_eq!(crc32(b""), 0);
    }

    #[test]
    fn crc32_known() {
        assert_eq!(crc32(b"123456789"), 0xCBF43926);
    }

    #[test]
    fn crc32_update_chaining() {
        let full = crc32(b"helloworld");
        let mut h = crc32fast::Hasher::new();
        h.update(b"hello");
        let mid = h.finalize();
        let chained = crc32_update(mid, b"world");
        assert_eq!(full, chained);
    }
}
