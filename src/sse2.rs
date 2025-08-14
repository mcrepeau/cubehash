#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(target_feature = "avx2"), not(feature = "force-scalar")))]
pub mod sse2_backend {
    use crate::{Backend, BLOCKSIZE, ROUNDS, CubeHashParams, rounds_for_rev};
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    pub struct SSE2 {
        x0: __m128i, x1: __m128i, x2: __m128i, x3: __m128i,
        x4: __m128i, x5: __m128i, x6: __m128i, x7: __m128i
    }

    impl SSE2 {
        #[inline(always)]
        unsafe fn rounds(&mut self) {
            let mut y0: __m128i;
            let mut y1: __m128i;
            let mut y2: __m128i;
            let mut y3: __m128i;

            for _ in 0..ROUNDS {
                self.x4 = _mm_add_epi32(self.x0, _mm_shuffle_epi32(self.x4, 0xb1));
                self.x5 = _mm_add_epi32(self.x1, _mm_shuffle_epi32(self.x5, 0xb1));
                self.x6 = _mm_add_epi32(self.x2, _mm_shuffle_epi32(self.x6, 0xb1));
                self.x7 = _mm_add_epi32(self.x3, _mm_shuffle_epi32(self.x7, 0xb1));

                y0 = self.x2; y1 = self.x3; y2 = self.x0; y3 = self.x1;
                self.x0 = _mm_xor_si128(_mm_slli_epi32(y0, 7), _mm_srli_epi32(y0, 25));
                self.x1 = _mm_xor_si128(_mm_slli_epi32(y1, 7), _mm_srli_epi32(y1, 25));
                self.x2 = _mm_xor_si128(_mm_slli_epi32(y2, 7), _mm_srli_epi32(y2, 25));
                self.x3 = _mm_xor_si128(_mm_slli_epi32(y3, 7), _mm_srli_epi32(y3, 25));
                self.x0 = _mm_xor_si128(self.x0, self.x4);
                self.x1 = _mm_xor_si128(self.x1, self.x5);
                self.x2 = _mm_xor_si128(self.x2, self.x6);
                self.x3 = _mm_xor_si128(self.x3, self.x7);

                self.x4 = _mm_add_epi32(self.x0, _mm_shuffle_epi32(self.x4, 0x4e));
                self.x5 = _mm_add_epi32(self.x1, _mm_shuffle_epi32(self.x5, 0x4e));
                self.x6 = _mm_add_epi32(self.x2, _mm_shuffle_epi32(self.x6, 0x4e));
                self.x7 = _mm_add_epi32(self.x3, _mm_shuffle_epi32(self.x7, 0x4e));
                y0 = self.x1; y1 = self.x0; y2 = self.x3; y3 = self.x2;
                self.x0 = _mm_xor_si128(_mm_slli_epi32(y0, 11), _mm_srli_epi32(y0, 21));
                self.x1 = _mm_xor_si128(_mm_slli_epi32(y1, 11), _mm_srli_epi32(y1, 21));
                self.x2 = _mm_xor_si128(_mm_slli_epi32(y2, 11), _mm_srli_epi32(y2, 21));
                self.x3 = _mm_xor_si128(_mm_slli_epi32(y3, 11), _mm_srli_epi32(y3, 21));
                self.x0 = _mm_xor_si128(self.x0, self.x4);
                self.x1 = _mm_xor_si128(self.x1, self.x5);
                self.x2 = _mm_xor_si128(self.x2, self.x6);
                self.x3 = _mm_xor_si128(self.x3, self.x7);
            }
        }
    }

    impl Backend for SSE2 {
        fn new(params: CubeHashParams) -> Self {
            unsafe {
                let (irounds, _frounds) = rounds_for_rev(params.revision);
                let mut st = SSE2 {
                    x0: _mm_set_epi32(0, ROUNDS, BLOCKSIZE as i32, params.hash_len_bits / 8),
                    x1: _mm_setzero_si128(),
                    x2: _mm_setzero_si128(),
                    x3: _mm_setzero_si128(),
                    x4: _mm_setzero_si128(),
                    x5: _mm_setzero_si128(),
                    x6: _mm_setzero_si128(),
                    x7: _mm_setzero_si128(),
                };
                for _ in 0..(irounds / ROUNDS) { st.rounds(); }
                st
            }
        }

        fn absorb_block(&mut self, block32: &[u8]) {
            unsafe {
                debug_assert_eq!(block32.len(), BLOCKSIZE);
                let p = block32.as_ptr() as *const __m128i;
                self.x0 = _mm_xor_si128(self.x0, _mm_loadu_si128(p));
                self.x1 = _mm_xor_si128(self.x1, _mm_loadu_si128(p.add(1)));
                self.rounds();
            }
        }

        fn set_finalize_flag(&mut self) {
            unsafe { self.x7 = _mm_xor_si128(self.x7, _mm_set_epi32(0, 1, 0, 0)); }
        }

        fn rounds_only(&mut self) { unsafe { self.rounds(); } }

        fn output_full(&self) -> [u8; 64] {
            unsafe {
                let x0: [u8; 16] = core::mem::transmute(self.x0);
                let x1: [u8; 16] = core::mem::transmute(self.x1);
                let x2: [u8; 16] = core::mem::transmute(self.x2);
                let x3: [u8; 16] = core::mem::transmute(self.x3);
                let mut out = [0u8; 64];
                out[0..16].copy_from_slice(&x0);
                out[16..32].copy_from_slice(&x1);
                out[32..48].copy_from_slice(&x2);
                out[48..64].copy_from_slice(&x3);
                out
            }
        }
    }
}

// Re-export at crate level
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(target_feature = "avx2"), not(feature = "force-scalar")))]
pub use sse2_backend::SSE2;
