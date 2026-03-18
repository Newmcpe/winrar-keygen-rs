use sha1::{Sha1, Digest};

pub fn sha1_of(data: &[u8]) -> [u8; 20] {
    let mut h = Sha1::new();
    h.update(data);
    h.finalize().into()
}

pub fn sha1_of_words(words: &[u32]) -> [u8; 20] {
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for &w in words {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    sha1_of(&bytes)
}

pub fn crc32(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}

pub fn crc32_update(crc: u32, data: &[u8]) -> u32 {
    let mut h = crc32fast::Hasher::new_with_initial(crc);
    h.update(data);
    h.finalize()
}
