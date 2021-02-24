#[derive(Clone, Copy)]
pub struct U32x4 {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32
}

impl U32x4 {
    pub fn load(data: &[u8]) -> U32x4 {
        U32x4 { a: as_u32_le(&data[12..16]), b: as_u32_le(&data[8..12]), c: as_u32_le(&data[4..8]), d: as_u32_le(&data[0..4]) }
    }

    pub fn permute_badc(self) -> U32x4 {
        U32x4 { a: self.b, b: self.a, c: self.d, d: self.c }
    }

    pub fn permute_cdab(self) -> U32x4 {
        U32x4 { a: self.c, b: self.d, c: self.a, d: self.b }
    }

    pub fn shift_left(self, mask: u32) -> U32x4 {
    U32x4 { a: self.a.wrapping_shl(mask), b: self.b.wrapping_shl(mask), c: self.c.wrapping_shl(mask), d: self.d.wrapping_shl(mask) }
}

    pub fn shift_right(self, mask: u32) -> U32x4 {
        U32x4 { a: self.a.wrapping_shr(mask), b: self.b.wrapping_shr(mask), c: self.c.wrapping_shr(mask), d: self.d.wrapping_shr(mask) }
    }

    pub fn transmute(self) -> Vec<u8> {
        [self.d.to_le_bytes(), self.c.to_le_bytes(), self.b.to_le_bytes(), self.a.to_le_bytes()].concat()
    }
}

fn as_u32_le(array: &[u8]) -> u32 {
    ((array[0] as u32) <<  0) +
    ((array[1] as u32) <<  8) +
    ((array[2] as u32) << 16) +
    ((array[3] as u32) << 24)
}

pub fn add(v: U32x4, w: U32x4) -> U32x4 {
    U32x4 { a: v.a.wrapping_add(w.a),
            b: v.b.wrapping_add(w.b),
            c: v.c.wrapping_add(w.c),
            d: v.d.wrapping_add(w.d) }
}


pub fn xor(v: U32x4, w: U32x4) -> U32x4 {
    U32x4 { a: v.a ^ w.a,
            b: v.b ^ w.b,
            c: v.c ^ w.c,
            d: v.d ^ w.d }
}