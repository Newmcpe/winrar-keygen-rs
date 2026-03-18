#[cfg(test)]
mod tests {
    use crate::ecc::*;
    use crate::gf::Gf2p15p17 as F;

    fn is_on_curve(p: &Point) -> bool {
        if p.infinity { return true; }
        let y2 = F::square(&p.y);
        let xy = F::mul(&p.x, &p.y);
        let lhs = F::add(&y2, &xy);
        let x3 = F::mul(&F::square(&p.x), &p.x);
        let b: [u16; 17] = {
            let mut b = [0u16; 17];
            b[0] = 161;
            b
        };
        let rhs = F::add(&x3, &b);
        F::eq(&lhs, &rhs)
    }

    #[test]
    fn base_point_on_curve() {
        assert!(is_on_curve(&BASE_POINT));
    }

    #[test]
    fn infinity_is_identity() {
        let sum = add_points(&INFINITY, &BASE_POINT);
        assert_eq!(sum.x, BASE_POINT.x);
        assert_eq!(sum.y, BASE_POINT.y);

        let sum2 = add_points(&BASE_POINT, &INFINITY);
        assert_eq!(sum2.x, BASE_POINT.x);
    }

    #[test]
    fn double_base_point_on_curve() {
        let p2 = double(&BASE_POINT);
        assert!(!p2.infinity);
        assert!(is_on_curve(&p2));
    }

    #[test]
    fn add_base_point_to_itself() {
        let doubled = double(&BASE_POINT);
        let added = add_points(&BASE_POINT, &BASE_POINT);
        assert_eq!(doubled.x, added.x);
        assert_eq!(doubled.y, added.y);
    }

    #[test]
    fn scalar_mul_one() {
        let one = [1u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let p = scalar_mul(&BASE_POINT, &one);
        assert_eq!(p.x, BASE_POINT.x);
        assert_eq!(p.y, BASE_POINT.y);
    }

    #[test]
    fn scalar_mul_two() {
        let two = [2u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let p2 = scalar_mul(&BASE_POINT, &two);
        let doubled = double(&BASE_POINT);
        assert_eq!(p2.x, doubled.x);
        assert_eq!(p2.y, doubled.y);
    }

    #[test]
    fn scalar_mul_three() {
        let three = [3u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let p3 = scalar_mul(&BASE_POINT, &three);
        assert!(is_on_curve(&p3));

        let two = [2u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let p2 = scalar_mul(&BASE_POINT, &two);
        let p2_plus_g = add_points(&p2, &BASE_POINT);
        assert_eq!(p3.x, p2_plus_g.x);
    }

    #[test]
    fn scalar_mul_result_on_curve() {
        let k = [0x1234u16, 0x5678, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let p = scalar_mul(&BASE_POINT, &k);
        assert!(is_on_curve(&p));
    }

    #[test]
    fn scalar_mul_additive_property() {
        let a = [100u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let b = [200u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let ab = [300u16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        let pa = scalar_mul(&BASE_POINT, &a);
        let pb = scalar_mul(&BASE_POINT, &b);
        let pab = scalar_mul(&BASE_POINT, &ab);
        let pa_pb = add_points(&pa, &pb);

        assert_eq!(pab.x, pa_pb.x);
        assert_eq!(pab.y, pa_pb.y);
    }

    #[test]
    fn scalar_mul_privkey_gives_known_pubkey() {
        let pk = crate::keygen::generate_private_key(&[]);
        let pub_pt = scalar_mul(&BASE_POINT, &pk);
        assert!(is_on_curve(&pub_pt));

        let expected_x: [u16; 17] = [
            0x3A1A, 0x1109, 0x268A, 0x12F7, 0x3734, 0x75F0, 0x576C, 0x2EA4,
            0x4813, 0x3F62, 0x0567, 0x784D, 0x753D, 0x6D92, 0x366C, 0x1107, 0x3861,
        ];
        assert_eq!(pub_pt.x, expected_x);
    }

    #[test]
    fn dump_x_as_le_u16s_length() {
        assert_eq!(dump_x_as_le_u16s(&BASE_POINT).len(), 15);
    }
}
