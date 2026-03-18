#[cfg(test)]
mod tests {
    use crate::gf::*;

    // ── GF(2^15) ─────────────────────────────────────────────────────────────

    #[test]
    fn gf15_add_is_xor() {
        assert_eq!(Gf2p15::add(0, 0), 0);
        assert_eq!(Gf2p15::add(0x1234, 0x1234), 0);
        assert_eq!(Gf2p15::add(0x00FF, 0xFF00), 0xFFFF);
    }

    #[test]
    fn gf15_mul_identity() {
        assert_eq!(Gf2p15::mul(1, 1), 1);
        assert_eq!(Gf2p15::mul(0x4321, 1), 0x4321);
        assert_eq!(Gf2p15::mul(1, 0x4321), 0x4321);
    }

    #[test]
    fn gf15_mul_zero() {
        assert_eq!(Gf2p15::mul(0, 0x7FFF), 0);
        assert_eq!(Gf2p15::mul(0x1234, 0), 0);
    }

    #[test]
    fn gf15_inv_one() {
        assert_eq!(Gf2p15::inv(1), 1);
    }

    #[test]
    fn gf15_inv_roundtrip() {
        for &a in &[1u16, 2, 3, 161, 0x7FFF, 0x4000, 1234] {
            assert_eq!(Gf2p15::mul(a, Gf2p15::inv(a)), 1, "a={a}");
        }
    }

    #[test]
    #[should_panic]
    fn gf15_inv_zero_panics() {
        Gf2p15::inv(0);
    }

    #[test]
    fn gf15_square_matches_mul() {
        for &a in &[0u16, 1, 2, 161, 0x7FFF] {
            assert_eq!(Gf2p15::square(a), Gf2p15::mul(a, a), "a={a}");
        }
    }

    #[test]
    fn gf15_div_roundtrip() {
        let a = 0x1234u16;
        let b = 0x5678u16;
        assert_eq!(Gf2p15::mul(Gf2p15::div(a, b), b), a);
    }

    // ── GF((2^15)^17) ────────────────────────────────────────────────────────

    #[test]
    fn gf17_zero_and_one() {
        assert!(Gf2p15p17::is_zero(&Gf2p15p17::zero()));
        assert!(Gf2p15p17::is_one(&Gf2p15p17::one()));
        assert!(!Gf2p15p17::is_zero(&Gf2p15p17::one()));
        assert!(!Gf2p15p17::is_one(&Gf2p15p17::zero()));
    }

    #[test]
    fn gf17_add_self_is_zero() {
        let a: Elem = [161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(Gf2p15p17::is_zero(&Gf2p15p17::add(&a, &a)));
    }

    #[test]
    fn gf17_add_assign_matches_add() {
        let a: Elem = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let b: Elem = [4, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected = Gf2p15p17::add(&a, &b);
        let mut c = a;
        Gf2p15p17::add_assign(&mut c, &b);
        assert_eq!(c, expected);
    }

    #[test]
    fn gf17_mul_identity() {
        let one = Gf2p15p17::one();
        let a: Elem = [161, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7];
        assert!(Gf2p15p17::eq(&Gf2p15p17::mul(&a, &one), &a));
        assert!(Gf2p15p17::eq(&Gf2p15p17::mul(&one, &a), &a));
    }

    #[test]
    fn gf17_mul_zero() {
        let z = Gf2p15p17::zero();
        let a: Elem = [161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert!(Gf2p15p17::is_zero(&Gf2p15p17::mul(&a, &z)));
    }

    #[test]
    fn gf17_square_matches_mul() {
        let a: Elem = [100, 200, 300, 400, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(Gf2p15p17::square(&a), Gf2p15p17::mul(&a, &a));
    }

    #[test]
    fn gf17_inv_roundtrip() {
        let a: Elem = [161, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let inv_a = Gf2p15p17::inv(&a);
        assert!(Gf2p15p17::is_one(&Gf2p15p17::mul(&a, &inv_a)));
    }

    #[test]
    fn gf17_inv_full_element() {
        let a: Elem = [14016, 2148, 26097, 20111, 31767, 30433, 29329, 31910,
                       26950, 7821, 6333, 31515, 26843, 23371, 17929, 998, 14976];
        let inv_a = Gf2p15p17::inv(&a);
        assert!(!Gf2p15p17::is_zero(&inv_a));
        assert!(Gf2p15p17::is_one(&Gf2p15p17::mul(&a, &inv_a)));
    }

    #[test]
    #[should_panic]
    fn gf17_inv_zero_panics() {
        Gf2p15p17::inv(&Gf2p15p17::zero());
    }

    #[test]
    fn gf17_div_roundtrip() {
        let a: Elem = [10, 20, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let b: Elem = [5, 15, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let q = Gf2p15p17::div(&a, &b);
        assert!(Gf2p15p17::eq(&Gf2p15p17::mul(&q, &b), &a));
    }

    #[test]
    fn gf17_dump_bytes_length() {
        assert_eq!(Gf2p15p17::dump_bytes(&Gf2p15p17::one()).len(), 32);
    }

    #[test]
    fn gf17_dump_bytes_zero() {
        assert!(Gf2p15p17::dump_bytes(&Gf2p15p17::zero()).iter().all(|&b| b == 0));
    }

    #[test]
    fn gf17_mul_commutativity() {
        let a: Elem = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
        let b: Elem = [17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(Gf2p15p17::mul(&a, &b), Gf2p15p17::mul(&b, &a));
    }

    #[test]
    fn gf17_mul_associativity() {
        let a: Elem = [100, 200, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let b: Elem = [300, 0, 400, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let c: Elem = [0, 500, 0, 600, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let ab_c = Gf2p15p17::mul(&Gf2p15p17::mul(&a, &b), &c);
        let a_bc = Gf2p15p17::mul(&a, &Gf2p15p17::mul(&b, &c));
        assert_eq!(ab_c, a_bc);
    }
}
