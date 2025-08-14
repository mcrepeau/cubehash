#[cfg(any(feature = "force-scalar", not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))))]
mod scalar_backend {
    use crate::{Backend, BLOCKSIZE, ROUNDS, CubeHashParams, rounds_for_rev};
    use std::convert::TryInto;

    #[derive(Clone, Copy)]
    struct U32x4 { a: u32, b: u32, c: u32, d: u32 }

    #[inline]
    fn add(x: U32x4, y: U32x4) -> U32x4 {
        U32x4 { 
            a: x.a.wrapping_add(y.a),
            b: x.b.wrapping_add(y.b),
            c: x.c.wrapping_add(y.c),
            d: x.d.wrapping_add(y.d),
        }
    }

    #[inline]
    fn xor(x: U32x4, y: U32x4) -> U32x4 {
        U32x4 { 
            a: x.a ^ y.a,
            b: x.b ^ y.b,
            c: x.c ^ y.c,
            d: x.d ^ y.d,
        }
    }

    impl U32x4 {
        #[inline] fn shift_left(self, n: u32) -> Self { U32x4 { a: self.a << n, b: self.b << n, c: self.c << n, d: self.d << n } }
        #[inline] fn shift_right(self, n: u32) -> Self { U32x4 { a: self.a >> n, b: self.b >> n, c: self.c >> n, d: self.d >> n } }
        #[inline] fn permute_badc(self) -> Self { U32x4 { a: self.b, b: self.a, c: self.d, d: self.c } }
        #[inline] fn permute_cdab(self) -> Self { U32x4 { a: self.c, b: self.d, c: self.a, d: self.b } }
        #[inline] fn load(bytes: &[u8]) -> Self {
            let a = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
            let b = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
            let c = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
            let d = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
            U32x4 { a, b, c, d }
        }
        #[inline] fn to_bytes(self) -> [u8; 16] {
            let mut out = [0u8; 16];
            out[0..4].copy_from_slice(&self.a.to_le_bytes());
            out[4..8].copy_from_slice(&self.b.to_le_bytes());
            out[8..12].copy_from_slice(&self.c.to_le_bytes());
            out[12..16].copy_from_slice(&self.d.to_le_bytes());
            out
        }
    }

    pub struct Scalar {
        x0: U32x4, x1: U32x4, x2: U32x4, x3: U32x4,
        x4: U32x4, x5: U32x4, x6: U32x4, x7: U32x4,
    }

    impl Scalar {
        #[inline(always)]
        fn rounds(&mut self) {
            let (mut y0, mut y1, mut y2, mut y3);
            for _ in 0..ROUNDS {
                self.x4 = add(self.x0, self.x4.permute_badc());
                self.x5 = add(self.x1, self.x5.permute_badc());
                self.x6 = add(self.x2, self.x6.permute_badc());
                self.x7 = add(self.x3, self.x7.permute_badc());

                y0 = self.x2; y1 = self.x3; y2 = self.x0; y3 = self.x1;
                self.x0 = xor(y0.shift_left(7), y0.shift_right(25));
                self.x1 = xor(y1.shift_left(7), y1.shift_right(25));
                self.x2 = xor(y2.shift_left(7), y2.shift_right(25));
                self.x3 = xor(y3.shift_left(7), y3.shift_right(25));

                self.x0 = xor(self.x0, self.x4);
                self.x1 = xor(self.x1, self.x5);
                self.x2 = xor(self.x2, self.x6);
                self.x3 = xor(self.x3, self.x7);

                self.x4 = add(self.x0, self.x4.permute_cdab());
                self.x5 = add(self.x1, self.x5.permute_cdab());
                self.x6 = add(self.x2, self.x6.permute_cdab());
                self.x7 = add(self.x3, self.x7.permute_cdab());

                y0 = self.x1; y1 = self.x0; y2 = self.x3; y3 = self.x2;
                self.x0 = xor(y0.shift_left(11), y0.shift_right(21));
                self.x1 = xor(y1.shift_left(11), y1.shift_right(21));
                self.x2 = xor(y2.shift_left(11), y2.shift_right(21));
                self.x3 = xor(y3.shift_left(11), y3.shift_right(21));

                self.x0 = xor(self.x0, self.x4);
                self.x1 = xor(self.x1, self.x5);
                self.x2 = xor(self.x2, self.x6);
                self.x3 = xor(self.x3, self.x7);
            }
        }
    }

    impl Backend for Scalar {
        fn new(params: CubeHashParams) -> Self {
            let (irounds, _frounds) = rounds_for_rev(params.revision);
            let mut st = Scalar {
                x0: U32x4 { a: 0, b: ROUNDS as u32, c: BLOCKSIZE as u32, d: (params.hash_len_bits / 8) as u32 },
                x1: U32x4 { a: 0, b: 0, c: 0, d: 0 },
                x2: U32x4 { a: 0, b: 0, c: 0, d: 0 },
                x3: U32x4 { a: 0, b: 0, c: 0, d: 0 },
                x4: U32x4 { a: 0, b: 0, c: 0, d: 0 },
                x5: U32x4 { a: 0, b: 0, c: 0, d: 0 },
                x6: U32x4 { a: 0, b: 0, c: 0, d: 0 },
                x7: U32x4 { a: 0, b: 0, c: 0, d: 0 },
            };
            for _ in 0..(irounds / ROUNDS) { st.rounds(); }
            st
        }

        fn absorb_block(&mut self, block32: &[u8]) {
            debug_assert_eq!(block32.len(), BLOCKSIZE);
            let m0 = U32x4::load(&block32[..16]);
            let m1 = U32x4::load(&block32[16..32]);
            self.x0 = xor(self.x0, m0);
            self.x1 = xor(self.x1, m1);
            self.rounds();
        }

        fn set_finalize_flag(&mut self) {
            self.x7 = xor(self.x7, U32x4 { a: 0, b: 1, c: 0, d: 0 });
        }

        fn rounds_only(&mut self) { self.rounds(); }

        fn output_full(&self) -> [u8; 64] {
            let mut out = [0u8; 64];
            out[0..16].copy_from_slice(&self.x0.to_bytes());
            out[16..32].copy_from_slice(&self.x1.to_bytes());
            out[32..48].copy_from_slice(&self.x2.to_bytes());
            out[48..64].copy_from_slice(&self.x3.to_bytes());
            out
        }
    }
}

// Re-export at crate level
#[cfg(any(feature = "force-scalar", not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))))]
pub use scalar_backend::Scalar;
