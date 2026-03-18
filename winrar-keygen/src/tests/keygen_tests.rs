#[cfg(test)]
mod tests {
    use crate::keygen::*;

    #[test]
    fn private_key_empty_seed_matches_known() {
        let pk = generate_private_key(&[]);
        assert_eq!(pk[0], 0xd65e);
        assert_eq!(pk[14], 0x59fe);
    }

    #[test]
    fn private_key_deterministic() {
        let a = generate_private_key(b"WinRAR");
        let b = generate_private_key(b"WinRAR");
        assert_eq!(a, b);
    }

    #[test]
    fn private_key_different_seeds() {
        let a = generate_private_key(b"Alice");
        let b = generate_private_key(b"Bob");
        assert_ne!(a, b);
    }

    #[test]
    fn public_key_sm2_deterministic() {
        let a = generate_public_key_sm2("WinRAR");
        let b = generate_public_key_sm2("WinRAR");
        assert_eq!(a, b);
    }

    #[test]
    fn public_key_sm2_length() {
        let pk = generate_public_key_sm2("test");
        assert_eq!(pk.len(), 64);
        assert!(pk.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn uid_matches_cpp_keygen() {
        let temp = generate_public_key_sm2("WinRAR");
        let items3 = format!("60{}", &temp[..48]);
        let items0 = generate_public_key_sm2(&items3);
        let uid = format!("{}{}", &temp[48..64], &items0[..4]);
        assert_eq!(uid, "be59262902d25ca06db3");
    }

    #[test]
    fn register_info_structure() {
        let info = generate_register_info("Test", "Test License");
        assert_eq!(info.username, "Test");
        assert_eq!(info.license_type, "Test License");
        assert_eq!(info.uid.len(), 20);
        assert_eq!(info.items[0].len(), 64);
        assert_eq!(info.items[1].len(), 122);
        assert_eq!(info.items[2].len(), 122);
        assert_eq!(info.items[3].len(), 50);
        assert_eq!(info.hex_data.len() % 54, 0);
    }

    #[test]
    fn register_info_items_start_with_60() {
        let info = generate_register_info("User", "License");
        assert!(info.items[1].starts_with("60"));
        assert!(info.items[2].starts_with("60"));
        assert!(info.items[3].starts_with("60"));
    }

    #[test]
    fn register_info_hex_data_starts_with_lengths() {
        let info = generate_register_info("User", "License");
        assert!(info.hex_data.starts_with("6412212250"));
    }

    #[test]
    fn license_text_has_crlf() {
        let text = generate_license_text("User", "License");
        assert!(text.contains("\r\n"));
        assert!(!text.contains("\r\n\r\n"));
    }

    #[test]
    fn license_text_starts_with_header() {
        let text = generate_license_text("User", "License");
        assert!(text.starts_with("RAR registration data\r\n"));
    }

    #[test]
    fn license_text_contains_uid() {
        let text = generate_license_text("User", "License");
        assert!(text.contains("UID="));
    }

    #[test]
    fn license_text_seven_data_lines() {
        let text = generate_license_text("User", "License");
        let lines: Vec<&str> = text.trim_end().split("\r\n").collect();
        assert_eq!(lines.len(), 11);
        for line in &lines[4..] {
            assert_eq!(line.len(), 54);
        }
    }

    #[test]
    fn checksum_is_complement() {
        let info = generate_register_info("A", "B");
        let mut h = crc32fast::Hasher::new();
        h.update(info.license_type.as_bytes());
        h.update(info.username.as_bytes());
        for item in &info.items { h.update(item.as_bytes()); }
        assert_eq!(info.checksum, !h.finalize());
    }
}
