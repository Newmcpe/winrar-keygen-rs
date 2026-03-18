use crate::gf::{Elem, Gf2p15p17 as F};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: Elem,
    pub y: Elem,
    pub infinity: bool,
}

pub const INFINITY: Point = Point { x: [0u16; 17], y: [0u16; 17], infinity: true };

pub const BASE_POINT: Point = Point {
    infinity: false,
    x: [0x38CC, 0x052F, 0x2510, 0x45AA, 0x1B89, 0x4468, 0x4882, 0x0D67,
        0x4FEB, 0x55CE, 0x0025, 0x4CB7, 0x0CC2, 0x59DC, 0x289E, 0x65E3, 0x56FD],
    y: [0x31A7, 0x65F2, 0x18C4, 0x3412, 0x7388, 0x54C1, 0x539B, 0x4A02,
        0x4D07, 0x12D6, 0x7911, 0x3B5E, 0x4F0E, 0x216F, 0x2BF2, 0x1974, 0x20DA],
};

pub fn double(p: &Point) -> Point {
    if p.infinity { return INFINITY; }
    let inv_x = F::inv(&p.x);
    let lam = F::add(&p.x, &F::mul(&p.y, &inv_x));
    let x3 = F::add(&F::square(&lam), &lam);
    let y3 = F::add(&F::add(&F::square(&p.x), &F::mul(&lam, &x3)), &x3);
    Point { x: x3, y: y3, infinity: false }
}

pub fn add_points(p: &Point, q: &Point) -> Point {
    if p.infinity { return *q; }
    if q.infinity { return *p; }
    let dx = F::add(&p.x, &q.x);
    if F::is_zero(&dx) {
        let dy = F::add(&p.y, &q.y);
        return if F::is_zero(&dy) { double(p) } else { INFINITY };
    }
    let dy = F::add(&p.y, &q.y);
    let lam = F::div(&dy, &dx);
    let x3 = F::add(&F::add(&F::add(&F::square(&lam), &lam), &p.x), &q.x);
    let y3 = F::add(&F::add(&F::mul(&lam, &F::add(&p.x, &x3)), &x3), &p.y);
    Point { x: x3, y: y3, infinity: false }
}

pub fn scalar_mul(p: &Point, k: &[u16; 15]) -> Point {
    let mut result = INFINITY;
    for limb_idx in (0..15).rev() {
        for bit in (0..16).rev() {
            result = double(&result);
            if (k[limb_idx] >> bit) & 1 == 1 {
                result = add_points(&result, p);
            }
        }
    }
    result
}

pub fn dump_x_as_le_u16s(p: &Point) -> [u16; 15] {
    let bytes = F::dump_bytes(&p.x);
    let mut out = [0u16; 15];
    for i in 0..15 {
        out[i] = u16::from_le_bytes([bytes[2 * i], bytes[2 * i + 1]]);
    }
    out
}
