use std::sync::OnceLock;

pub type Elem15 = u16;
pub type Elem = [u16; 17];

// ── GF(2^15) ─────────────────────────────────────────────────────────────────
// Irreducible poly: x^15 + x + 1  (0x8003)

struct Gf2p15Tables {
    log: [u16; 32768],
    exp: [u16; 32768],
}

static GF15_TABLES: OnceLock<Gf2p15Tables> = OnceLock::new();

#[inline]
fn gf15_tables() -> &'static Gf2p15Tables {
    GF15_TABLES.get_or_init(|| {
        let mut log = [0u16; 32768];
        let mut exp = [0u16; 32768];
        exp[0] = 1;
        for i in 1..32767usize {
            let t = exp[i - 1] as u32 * 2;
            exp[i] = if t >= 0x8000 { (t ^ 0x8003) as u16 } else { t as u16 };
        }
        exp[32767] = 1; // g^ORDER = g^0 = 1
        for i in 0..32767usize {
            log[exp[i] as usize] = i as u16;
        }
        Gf2p15Tables { log, exp }
    })
}

pub struct Gf2p15;

impl Gf2p15 {
    #[inline]
    pub fn add(a: Elem15, b: Elem15) -> Elem15 { a ^ b }

    #[inline(always)]
    pub fn mul(a: Elem15, b: Elem15) -> Elem15 {
        if a == 0 || b == 0 { return 0; }
        let t = gf15_tables();
        let g = t.log[a as usize] as u32 + t.log[b as usize] as u32;
        let g = if g >= 32767 { g - 32767 } else { g };
        t.exp[g as usize]
    }

    #[inline(always)]
    pub fn inv(a: Elem15) -> Elem15 {
        assert_ne!(a, 0, "inv(0) undefined");
        let t = gf15_tables();
        let g = 32767u32 - t.log[a as usize] as u32;
        t.exp[g as usize]
    }

    #[inline(always)]
    pub fn square(a: Elem15) -> Elem15 { Self::mul(a, a) }
    #[inline(always)]
    pub fn div(a: Elem15, b: Elem15) -> Elem15 { Self::mul(a, Self::inv(b)) }
}

// ── GF((2^15)^17) ─────────────────────────────────────────────────────────────
// Irreducible poly: y^17 + y^3 + 1

pub struct Gf2p15p17;

impl Gf2p15p17 {
    #[inline]
    pub fn zero() -> Elem { [0u16; 17] }
    #[inline]
    pub fn one() -> Elem { let mut e = [0u16; 17]; e[0] = 1; e }

    #[inline]
    pub fn is_zero(a: &Elem) -> bool { a.iter().all(|&x| x == 0) }
    #[inline]
    pub fn is_one(a: &Elem) -> bool { a[0] == 1 && a[1..].iter().all(|&x| x == 0) }
    #[inline]
    pub fn eq(a: &Elem, b: &Elem) -> bool { a == b }

    #[inline]
    pub fn add(a: &Elem, b: &Elem) -> Elem {
        let mut r = *a;
        for i in 0..17 { r[i] ^= b[i]; }
        r
    }

    #[inline]
    pub fn add_assign(a: &mut Elem, b: &Elem) {
        for i in 0..17 { a[i] ^= b[i]; }
    }

    #[inline]
    pub fn mul(a: &Elem, b: &Elem) -> Elem {
        let mut temp = [0u16; 33];
        for i in 0..17 {
            if a[i] == 0 { continue; }
            for j in 0..17 {
                if b[j] != 0 {
                    temp[i + j] ^= Gf2p15::mul(a[i], b[j]);
                }
            }
        }
        Self::reduce33(&mut temp);
        temp[..17].try_into().unwrap()
    }

    #[inline]
    fn reduce33(temp: &mut [u16; 33]) {
        for i in (17..33).rev() {
            if temp[i] != 0 {
                let v = temp[i];
                temp[i - 17] ^= v;
                temp[i - 14] ^= v; // i-17+3
                temp[i] = 0;
            }
        }
    }

    #[inline]
    pub fn square(a: &Elem) -> Elem {
        let t = gf15_tables();
        let mut temp = [0u16; 33];
        for i in 0..17 {
            if a[i] != 0 {
                let g = (t.log[a[i] as usize] as u32 * 2) % 32767;
                temp[2 * i] = t.exp[g as usize];
            }
        }
        Self::reduce33(&mut temp);
        temp[..17].try_into().unwrap()
    }

    pub fn inv(a: &Elem) -> Elem {
        // Extended Euclidean over GF(2^15)[y] polynomials
        // F = a (copy), G = y^17+y^3+1, B = 1, C = 0
        let mut f = [0u16; 34];
        let mut g = [0u16; 34];
        let mut b = [0u16; 34];
        let mut c = [0u16; 34];

        // Copy a into f, find degF
        let mut deg_f = 0usize;
        for i in 0..17 {
            f[i] = a[i];
            if a[i] != 0 { deg_f = i; }
        }
        assert!(!a.iter().all(|&x| x == 0), "inv(0) undefined");

        // G = y^17 + y^3 + 1
        g[0] = 1; g[3] = 1; g[17] = 1;
        let mut deg_g = 17usize;

        // B = 1
        b[0] = 1;
        let mut deg_b = 0usize;
        let mut deg_c = 0usize;

        loop {
            if deg_f == 0 {
                // result = B / f[0]
                let inv_f0 = Gf2p15::inv(f[0]);
                let mut result = [0u16; 17];
                for i in 0..=deg_b {
                    result[i] = Gf2p15::mul(b[i], inv_f0);
                }
                return result;
            }

            // Ensure degF >= degG
            if deg_f < deg_g {
                f.swap_with_slice(&mut g);
                b.swap_with_slice(&mut c);
                std::mem::swap(&mut deg_f, &mut deg_g);
                std::mem::swap(&mut deg_b, &mut deg_c);
            }

            let j = deg_f - deg_g;
            let alpha = Gf2p15::div(f[deg_f], g[deg_g]);

            // F += alpha * x^j * G
            for i in 0..=deg_g {
                if g[i] != 0 {
                    f[i + j] ^= Gf2p15::mul(alpha, g[i]);
                }
            }
            // B += alpha * x^j * C
            let max_bc = deg_c + j;
            for i in 0..=deg_c {
                if c[i] != 0 {
                    b[i + j] ^= Gf2p15::mul(alpha, c[i]);
                }
            }
            if max_bc > deg_b { deg_b = max_bc; }

            // Recompute degF
            while deg_f > 0 && f[deg_f] == 0 { deg_f -= 1; }
            while deg_b > 0 && b[deg_b] == 0 { deg_b -= 1; }
        }
    }

    #[inline]
    pub fn div(a: &Elem, b: &Elem) -> Elem {
        Self::mul(a, &Self::inv(b))
    }

    /// Pack 17×15-bit values into 32 bytes (LSB-first within each byte).
    /// Mirrors WinRarConfig.hpp Dump() exactly.
    #[inline]
    pub fn dump_bytes(a: &Elem) -> [u8; 32] {
        let mut out = [0u8; 32];
        let mut write_ptr = 0usize;
        let mut left_bits: u32 = 8;

        for i in 0..17 {
            let low8  = (a[i] & 0xFF) as u8;
            let high7 = ((a[i] >> 8) & 0x7F) as u8;

            // write low8
            if left_bits == 8 {
                out[write_ptr] = low8;
                write_ptr += 1;
            } else {
                out[write_ptr] |= low8 << (8 - left_bits);
                write_ptr += 1;
                out[write_ptr] = low8 >> left_bits;
            }

            // write high7
            if left_bits == 8 {
                out[write_ptr] = high7;
                left_bits = 1;
            } else if left_bits == 7 {
                out[write_ptr] |= high7 << 1;
                write_ptr += 1;
                left_bits = 8;
            } else {
                out[write_ptr] |= high7 << (8 - left_bits);
                write_ptr += 1;
                if write_ptr < 32 {
                    out[write_ptr] = high7 >> left_bits;
                }
                left_bits = 8 - (7 - left_bits);
            }
        }
        out
    }
}
