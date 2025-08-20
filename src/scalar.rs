#[cfg(any(feature = "force-scalar", not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))))]
mod scalar_backend {
    use crate::{Backend, BLOCKSIZE, ROUNDS, CubeHashParams, rounds_for_rev};

    #[derive(Clone, Copy)]
    struct U32x4([u32; 4]);

    #[inline(always)]
    fn add(v: U32x4, w: U32x4) -> U32x4 {
        U32x4([
            v.0[0].wrapping_add(w.0[0]),
            v.0[1].wrapping_add(w.0[1]),
            v.0[2].wrapping_add(w.0[2]),
            v.0[3].wrapping_add(w.0[3]),
        ])
    }

    #[inline(always)]
    fn xor(v: U32x4, w: U32x4) -> U32x4 {
        U32x4([v.0[0] ^ w.0[0], v.0[1] ^ w.0[1], v.0[2] ^ w.0[2], v.0[3] ^ w.0[3]])
    }

    #[inline(always)]
    fn shlxor(v: U32x4, n: u32) -> U32x4 {
        let [a,b,c,d] = v.0;
        U32x4([
            (a.wrapping_shl(n)) ^ (a.wrapping_shr(32 - n)),
            (b.wrapping_shl(n)) ^ (b.wrapping_shr(32 - n)),
            (c.wrapping_shl(n)) ^ (c.wrapping_shr(32 - n)),
            (d.wrapping_shl(n)) ^ (d.wrapping_shr(32 - n)),
        ])
    }

    #[inline]
    fn as_u32_le(array: &[u8]) -> u32 {
            ((array[0] as u32) <<  0) +
            ((array[1] as u32) <<  8) +
            ((array[2] as u32) << 16) +
            ((array[3] as u32) << 24)
    }

    impl U32x4 {
        #[inline(always)]
        fn new(a: u32, b: u32, c: u32, d: u32) -> Self {
            U32x4([a, b, c, d])
        }

        #[inline(always)]
        fn permute_badc(self) -> U32x4 {
            U32x4([self.0[1], self.0[0], self.0[3], self.0[2]])
        }

        #[inline(always)]
        fn permute_cdab(self) -> U32x4 {
            U32x4([self.0[2], self.0[3], self.0[0], self.0[1]])
        }

        #[inline(always)]
        fn load_bytes(data: &[u8]) -> U32x4 {
            U32x4([
                as_u32_le(&data[12..16]),
                as_u32_le(&data[8..12]),
                as_u32_le(&data[4..8]),
                as_u32_le(&data[0..4])
            ])
        }

        #[inline]
        pub fn transmute(self) -> Vec<u8> {
            [self.0[3].to_le_bytes(), self.0[2].to_le_bytes(), self.0[1].to_le_bytes(), self.0[0].to_le_bytes()].concat()
        }
    }

    pub struct Scalar {
        x0: U32x4, x1: U32x4, x2: U32x4, x3: U32x4,
        x4: U32x4, x5: U32x4, x6: U32x4, x7: U32x4,
    }

    impl Scalar {
        #[inline(always)]
        fn rounds(&mut self) {
            for _ in 0..ROUNDS {
                self.x4 = add(self.x0, self.x4.permute_badc());
                self.x5 = add(self.x1, self.x5.permute_badc());
                self.x6 = add(self.x2, self.x6.permute_badc());
                self.x7 = add(self.x3, self.x7.permute_badc());

                let t0 = shlxor(self.x2, 7);
                let t1 = shlxor(self.x3, 7);
                let t2 = shlxor(self.x0, 7);
                let t3 = shlxor(self.x1, 7);

                self.x0 = xor(t0, self.x4);
                self.x1 = xor(t1, self.x5);
                self.x2 = xor(t2, self.x6);
                self.x3 = xor(t3, self.x7);

                self.x4 = add(self.x0, self.x4.permute_cdab());
                self.x5 = add(self.x1, self.x5.permute_cdab());
                self.x6 = add(self.x2, self.x6.permute_cdab());
                self.x7 = add(self.x3, self.x7.permute_cdab());

                let u0 = shlxor(self.x1, 11);
                let u1 = shlxor(self.x0, 11);
                let u2 = shlxor(self.x3, 11);
                let u3 = shlxor(self.x2, 11);

                self.x0 = xor(u0, self.x4);
                self.x1 = xor(u1, self.x5);
                self.x2 = xor(u2, self.x6);
                self.x3 = xor(u3, self.x7);
            }
        }
    }

    impl Backend for Scalar {
        fn new(params: CubeHashParams) -> Self {
            let (irounds, _frounds) = rounds_for_rev(params.revision);
            let mut st = Scalar {
                x0: U32x4::new(0, ROUNDS as u32, BLOCKSIZE as u32, (params.hash_len_bits / 8) as u32),
                x1: U32x4::new(0, 0, 0, 0),
                x2: U32x4::new(0, 0, 0, 0),
                x3: U32x4::new(0, 0, 0, 0),
                x4: U32x4::new(0, 0, 0, 0),
                x5: U32x4::new(0, 0, 0, 0),
                x6: U32x4::new(0, 0, 0, 0),
                x7: U32x4::new(0, 0, 0, 0),
            };
            for _ in 0..(irounds / ROUNDS) { st.rounds(); }
            st
        }

        fn absorb_block(&mut self, block32: &[u8]) {
            debug_assert_eq!(block32.len(), BLOCKSIZE);
            let m0 = U32x4::load_bytes(&block32[..16]);
            let m1 = U32x4::load_bytes(&block32[16..32]);
            self.x0 = xor(self.x0, m0);
            self.x1 = xor(self.x1, m1);
            self.rounds();
        }

        fn set_finalize_flag(&mut self) {
            self.x7 = xor(self.x7, U32x4::new(0, 1, 0, 0));
        }

        fn rounds_only(&mut self) { self.rounds(); }

        fn output_full(&self) -> [u8; 64] {
            let mut out = [0u8; 64];
            out[0..16].copy_from_slice(&self.x0.transmute());
            out[16..32].copy_from_slice(&self.x1.transmute());
            out[32..48].copy_from_slice(&self.x2.transmute());
            out[48..64].copy_from_slice(&self.x3.transmute());
            out
        }
    }
}

// Re-export at crate level
#[cfg(any(feature = "force-scalar", not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))))]
pub use scalar_backend::Scalar;
