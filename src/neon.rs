#[cfg(all(target_arch = "aarch64", not(feature = "force-scalar")))]
pub mod neon {
    use crate::{Backend, BLOCKSIZE, ROUNDS, CubeHashParams, rounds_for_rev};
    use core::arch::aarch64::*;

    #[inline(always)]
    unsafe fn load_le_u32x4_reversed(bytes: &[u8]) -> uint32x4_t {
        // matches your helper: d,c,b,a order to match intrinsic behavior
        let a = u32::from_le_bytes(bytes[12..16].try_into().unwrap());
        let b = u32::from_le_bytes(bytes[8..12].try_into().unwrap());
        let c = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
        let d = u32::from_le_bytes(bytes[0..4].try_into().unwrap());
        vld1q_u32([a, b, c, d].as_ptr())
    }
    #[inline(always)] unsafe fn to_bytes_d_c_b_a(v: uint32x4_t) -> [u8; 16] {
        let mut arr = [0u32; 4]; vst1q_u32(arr.as_mut_ptr(), v);
        let bytes = [arr[3].to_le_bytes(), arr[2].to_le_bytes(), arr[1].to_le_bytes(), arr[0].to_le_bytes()];
        let mut out = [0u8; 16];
        out[0..4].copy_from_slice(&bytes[0]);
        out[4..8].copy_from_slice(&bytes[1]);
        out[8..12].copy_from_slice(&bytes[2]);
        out[12..16].copy_from_slice(&bytes[3]);
        out
    }
    #[inline(always)] unsafe fn permute_badc(v: uint32x4_t) -> uint32x4_t { vrev64q_u32(v) }
    #[inline(always)] unsafe fn permute_cdab(v: uint32x4_t) -> uint32x4_t { vextq_u32::<2>(v, v) }

    pub struct NEON {
        x0: uint32x4_t, x1: uint32x4_t, x2: uint32x4_t, x3: uint32x4_t,
        x4: uint32x4_t, x5: uint32x4_t, x6: uint32x4_t, x7: uint32x4_t
    }

    impl NEON {
        #[inline(always)]
        unsafe fn rounds(&mut self) {
            let mut y0: uint32x4_t;
            let mut y1: uint32x4_t;
            let mut y2: uint32x4_t;
            let mut y3: uint32x4_t;

            for _ in 0..ROUNDS {
                self.x4 = vaddq_u32(self.x0, permute_badc(self.x4));
                self.x5 = vaddq_u32(self.x1, permute_badc(self.x5));
                self.x6 = vaddq_u32(self.x2, permute_badc(self.x6));
                self.x7 = vaddq_u32(self.x3, permute_badc(self.x7));

                y0 = self.x2; y1 = self.x3; y2 = self.x0; y3 = self.x1;
                self.x0 = veorq_u32(vshlq_n_u32::<7>(y0), vshrq_n_u32::<25>(y0));
                self.x1 = veorq_u32(vshlq_n_u32::<7>(y1), vshrq_n_u32::<25>(y1));
                self.x2 = veorq_u32(vshlq_n_u32::<7>(y2), vshrq_n_u32::<25>(y2));
                self.x3 = veorq_u32(vshlq_n_u32::<7>(y3), vshrq_n_u32::<25>(y3));
                self.x0 = veorq_u32(self.x0, self.x4);
                self.x1 = veorq_u32(self.x1, self.x5);
                self.x2 = veorq_u32(self.x2, self.x6);
                self.x3 = veorq_u32(self.x3, self.x7);

                self.x4 = vaddq_u32(self.x0, permute_cdab(self.x4));
                self.x5 = vaddq_u32(self.x1, permute_cdab(self.x5));
                self.x6 = vaddq_u32(self.x2, permute_cdab(self.x6));
                self.x7 = vaddq_u32(self.x3, permute_cdab(self.x7));

                y0 = self.x1; y1 = self.x0; y2 = self.x3; y3 = self.x2;
                self.x0 = veorq_u32(vshlq_n_u32::<11>(y0), vshrq_n_u32::<21>(y0));
                self.x1 = veorq_u32(vshlq_n_u32::<11>(y1), vshrq_n_u32::<21>(y1));
                self.x2 = veorq_u32(vshlq_n_u32::<11>(y2), vshrq_n_u32::<21>(y2));
                self.x3 = veorq_u32(vshlq_n_u32::<11>(y3), vshrq_n_u32::<21>(y3));
                self.x0 = veorq_u32(self.x0, self.x4);
                self.x1 = veorq_u32(self.x1, self.x5);
                self.x2 = veorq_u32(self.x2, self.x6);
                self.x3 = veorq_u32(self.x3, self.x7);
            }
        }
    }

    impl Backend for NEON {
        fn new(params: CubeHashParams) -> Self {
            unsafe {
                let (irounds, _frounds) = rounds_for_rev(params.revision);
                let mut st = NEON {
                    x0: vld1q_u32([0u32, ROUNDS as u32, BLOCKSIZE as u32, (params.hash_len_bits / 8) as u32].as_ptr()),
                    x1: vdupq_n_u32(0),
                    x2: vdupq_n_u32(0),
                    x3: vdupq_n_u32(0),
                    x4: vdupq_n_u32(0),
                    x5: vdupq_n_u32(0),
                    x6: vdupq_n_u32(0),
                    x7: vdupq_n_u32(0),
                };
                for _ in 0..(irounds / ROUNDS) { st.rounds(); }
                st
            }
        }

        fn absorb_block(&mut self, block32: &[u8]) {
            unsafe {
                debug_assert_eq!(block32.len(), BLOCKSIZE);
                self.x0 = veorq_u32(self.x0, load_le_u32x4_reversed(&block32[..16]));
                self.x1 = veorq_u32(self.x1, load_le_u32x4_reversed(&block32[16..32]));
                self.rounds();
            }
        }

        fn set_finalize_flag(&mut self) {
            unsafe { self.x7 = veorq_u32(self.x7, vld1q_u32([0u32, 1, 0, 0].as_ptr())); }
        }

        fn rounds_only(&mut self) { unsafe { self.rounds(); } }

        fn output_full(&self) -> [u8; 64] {
            unsafe {
                let mut out = [0u8; 64];
                out[0..16].copy_from_slice(&to_bytes_d_c_b_a(self.x0));
                out[16..32].copy_from_slice(&to_bytes_d_c_b_a(self.x1));
                out[32..48].copy_from_slice(&to_bytes_d_c_b_a(self.x2));
                out[48..64].copy_from_slice(&to_bytes_d_c_b_a(self.x3));
                out
            }
        }
    }
}

// Re-export at crate level
#[cfg(all(target_arch = "aarch64", not(feature = "force-scalar")))]
pub use neon_backend::NEON;
