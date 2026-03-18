use std::fmt::Write;
use std::sync::OnceLock;

use num_bigint::BigUint;
use num_traits::Zero;
use rand::random;

use crate::crypto::{sha1_of, sha1_of_words};
use crate::ecc::{scalar_mul, BASE_POINT};
use crate::gf::Gf2p15p17 as F;

pub struct RegisterInfo {
    pub username: String,
    pub license_type: String,
    pub uid: String,
    pub items: [String; 4],
    pub checksum: u32,
    pub hex_data: String,
}

fn order() -> &'static BigUint {
    static V: OnceLock<BigUint> = OnceLock::new();
    V.get_or_init(|| BigUint::parse_bytes(b"1026dd85081b82314691ced9bbec30547840e4bf72d8b5e0d258442bbcd31", 16).unwrap())
}

fn private_key() -> &'static BigUint {
    static V: OnceLock<BigUint> = OnceLock::new();
    V.get_or_init(|| BigUint::parse_bytes(b"59fe6abcca90bdb95f0105271fa85fb9f11f467450c1ae9044b7fd61d65e", 16).unwrap())
}

pub fn generate_private_key(seed: &[u8]) -> [u16; 15] {
    let gen: [u32; 5] = if seed.is_empty() {
        [0xeb3eb781, 0x50265329, 0xdc5ef4a3, 0x6847b9d5, 0xcde43b4c]
    } else {
        let d = sha1_of(seed);
        std::array::from_fn(|i| u32::from_be_bytes(d[4 * i..4 * i + 4].try_into().unwrap()))
    };

    std::array::from_fn(|i| {
        let input = [(i + 1) as u32, gen[0], gen[1], gen[2], gen[3], gen[4]];
        let d = sha1_of_words(&input);
        u32::from_be_bytes(d[0..4].try_into().unwrap()) as u16
    })
}

pub fn generate_public_key_sm2(message: &str) -> String {
    let priv_limbs = generate_private_key(message.as_bytes());
    let pt = scalar_mul(&BASE_POINT, &priv_limbs);

    let mut x_int = BigUint::from_bytes_le(&F::dump_bytes(&pt.x));
    x_int <<= 1;
    if F::div(&pt.y, &pt.x)[0] & 1 == 1 {
        x_int += 1u32;
    }

    let hex = format!("{:064x}", x_int);
    hex[hex.len().saturating_sub(64)..].to_string()
}

fn hash_integer(data: &[u8]) -> BigUint {
    let d = sha1_of(data);
    let mut all: [u32; 10] = [0; 10];
    for i in 0..5 {
        all[i] = u32::from_be_bytes(d[4 * i..4 * i + 4].try_into().unwrap());
    }
    all[5..].copy_from_slice(&[0x0ffd8d43, 0xb4e33c7c, 0x53461bd1, 0x0f27a546, 0x1050d90d]);

    let mut bytes = [0u8; 40];
    for (i, &w) in all.iter().enumerate() {
        bytes[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
    }
    BigUint::from_bytes_le(&bytes[..30])
}

fn sign(data: &[u8]) -> (BigUint, BigUint) {
    let n = order();
    let d = private_key();
    let hash = hash_integer(data);

    loop {
        let k_limbs: [u16; 15] = std::array::from_fn(|_| random::<u16>());
        let mut k_bytes = [0u8; 30];
        for i in 0..15 {
            k_bytes[2 * i..2 * i + 2].copy_from_slice(&k_limbs[i].to_le_bytes());
        }
        let k = BigUint::from_bytes_le(&k_bytes);
        if k.is_zero() { continue; }

        let kg = scalar_mul(&BASE_POINT, &k_limbs);
        if kg.infinity { continue; }

        let rx = BigUint::from_bytes_le(&F::dump_bytes(&kg.x));
        let r = (rx + &hash) % n;
        if r.is_zero() || (&r + &k) == *n { continue; }

        let dr = (d * &r) % n;
        let s = if k >= dr { (&k - &dr) % n } else { (&k + n - &dr) % n };
        if s.is_zero() || r.bits() > 240 || s.bits() > 240 { continue; }

        return (r, s);
    }
}

pub fn generate_register_info(username: &str, license_type: &str) -> RegisterInfo {
    let mut items: [String; 4] = Default::default();

    let temp = generate_public_key_sm2(username);
    items[3] = format!("60{}", &temp[..48]);
    items[0] = generate_public_key_sm2(&items[3]);
    let uid = format!("{}{}", &temp[48..64], &items[0][..4]);

    for (idx, msg) in [(1, license_type.to_string()), (2, format!("{}{}", username, items[0]))] {
        loop {
            let (r, s) = sign(msg.as_bytes());
            let r_hex = format!("{:060x}", r);
            let s_hex = format!("{:060x}", s);
            if r_hex.len() == 60 && s_hex.len() == 60 {
                items[idx] = format!("60{}{}", s_hex, r_hex);
                break;
            }
        }
    }

    let mut h = crc32fast::Hasher::new();
    h.update(license_type.as_bytes());
    h.update(username.as_bytes());
    for item in &items { h.update(item.as_bytes()); }
    let checksum = !h.finalize();

    let hex_data = format!(
        "{}{}{}{}{}{}{}{}{:010}",
        items[0].len(), items[1].len(), items[2].len(), items[3].len(),
        items[0], items[1], items[2], items[3], checksum
    );
    assert_eq!(hex_data.len() % 54, 0);

    RegisterInfo {
        username: username.to_string(),
        license_type: license_type.to_string(),
        uid, items, checksum, hex_data,
    }
}

pub fn generate_license_text(username: &str, license_type: &str) -> String {
    let info = generate_register_info(username, license_type);
    let mut out = String::with_capacity(512);
    let _ = write!(out, "RAR registration data\r\n{}\r\n{}\r\nUID={}\r\n",
        info.username, info.license_type, info.uid);
    for chunk in info.hex_data.as_bytes().chunks(54) {
        out.push_str(std::str::from_utf8(chunk).unwrap());
        out.push_str("\r\n");
    }
    out
}
