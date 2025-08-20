#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2", not(feature = "force-scalar")))]
pub mod avx2_backend {
    use crate::{Backend, BLOCKSIZE, ROUNDS, CubeHashParams, rounds_for_rev};
    #[cfg(target_arch = "x86")]
    use core::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64::*;

    pub struct AVX2 {
        v01: __m256i, v23: __m256i, v45: __m256i, v67: __m256i,
        zero128: __m128i,
    }

    impl AVX2 {
        #[inline(always)]
        unsafe fn rounds(&mut self) {
            for _ in 0..ROUNDS {
                // x4.. add & shuffle badc
                let sh_v45 = _mm256_shuffle_epi32(self.v45, 0xb1);
                self.v45 = _mm256_add_epi32(self.v01, sh_v45);
                let sh_v67 = _mm256_shuffle_epi32(self.v67, 0xb1);
                self.v67 = _mm256_add_epi32(self.v23, sh_v67);

                // rotate and swap pairs
                let t01 = self.v01;
                let t23 = self.v23;
                let rot_t23 = _mm256_xor_si256(_mm256_slli_epi32(t23, 7), _mm256_srli_epi32(t23, 25));
                let rot_t01 = _mm256_xor_si256(_mm256_slli_epi32(t01, 7), _mm256_srli_epi32(t01, 25));
                self.v01 = rot_t23;
                self.v23 = rot_t01;

                self.v01 = _mm256_xor_si256(self.v01, self.v45);
                self.v23 = _mm256_xor_si256(self.v23, self.v67);

                let sh2_v45 = _mm256_shuffle_epi32(self.v45, 0x4e);
                self.v45 = _mm256_add_epi32(self.v01, sh2_v45);
                let sh2_v67 = _mm256_shuffle_epi32(self.v67, 0x4e);
                self.v67 = _mm256_add_epi32(self.v23, sh2_v67);

                let r01 = _mm256_xor_si256(_mm256_slli_epi32(self.v01, 11), _mm256_srli_epi32(self.v01, 21));
                let r23 = _mm256_xor_si256(_mm256_slli_epi32(self.v23, 11), _mm256_srli_epi32(self.v23, 21));
                self.v01 = _mm256_permute2x128_si256(r01, r01, 0x01);
                self.v23 = _mm256_permute2x128_si256(r23, r23, 0x01);

                self.v01 = _mm256_xor_si256(self.v01, self.v45);
                self.v23 = _mm256_xor_si256(self.v23, self.v67);
            }
        }
    }

    impl Backend for AVX2 {
        fn new(params: CubeHashParams) -> Self {
            unsafe {
                let (irounds, _frounds) = rounds_for_rev(params.revision);
                let low_init: __m128i = _mm_set_epi32(0, ROUNDS, BLOCKSIZE as i32, params.hash_len_bits / 8);
                let zero128 = _mm_setzero_si128();

                let mut v01: __m256i = _mm256_castsi128_si256(low_init);
                v01 = _mm256_inserti128_si256(v01, zero128, 1);

                let mut st = AVX2 {
                    v01, v23: _mm256_setzero_si256(),
                    v45: _mm256_setzero_si256(), v67: _mm256_setzero_si256(),
                    zero128,
                };
                for _ in 0..(irounds / ROUNDS) { st.rounds(); }
                st
            }
        }

        fn absorb_block(&mut self, block32: &[u8]) {
            unsafe {
                debug_assert_eq!(block32.len(), BLOCKSIZE);
                let block = _mm256_loadu_si256(block32.as_ptr() as *const __m256i);
                self.v01 = _mm256_xor_si256(self.v01, block);
                self.rounds();
            }
        }

        fn set_finalize_flag(&mut self) {
            unsafe {
                let mut flag = _mm256_castsi128_si256(self.zero128);
                let one_mask: __m128i = _mm_set_epi32(0, 1, 0, 0);
                flag = _mm256_inserti128_si256(flag, one_mask, 1);
                self.v67 = _mm256_xor_si256(self.v67, flag);
            }
        }

        fn rounds_only(&mut self) { unsafe { self.rounds(); } }

        fn output_full(&self) -> [u8; 64] {
            unsafe {
                let x0: __m128i = _mm256_castsi256_si128(self.v01);
                let x1: __m128i = _mm256_extracti128_si256(self.v01, 1);
                let x2: __m128i = _mm256_castsi256_si128(self.v23);
                let x3: __m128i = _mm256_extracti128_si256(self.v23, 1);
                let xb0: [u8; 16] = core::mem::transmute(x0);
                let xb1: [u8; 16] = core::mem::transmute(x1);
                let xb2: [u8; 16] = core::mem::transmute(x2);
                let xb3: [u8; 16] = core::mem::transmute(x3);
                let mut out = [0u8; 64];
                out[0..16].copy_from_slice(&xb0);
                out[16..32].copy_from_slice(&xb1);
                out[32..48].copy_from_slice(&xb2);
                out[48..64].copy_from_slice(&xb3);
                out
            }
        }
    }
}

// Re-export at crate level
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2", not(feature = "force-scalar")))]
pub use avx2_backend::AVX2;
