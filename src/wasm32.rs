#[cfg(all(target_arch = "wasm32"))]
mod wasm32_backend {
    use core::arch::wasm32::*;
    use crate::{Backend, BLOCKSIZE, ROUNDS, CubeHashParams, rounds_for_rev};

    #[inline(always)]
    unsafe fn add32(a: v128, b: v128) -> v128 { i32x4_add(a, b) }

    #[inline(always)]
    unsafe fn xor128(a: v128, b: v128) -> v128 { v128_xor(a, b) }

    // Rotate-left by n on u32 lanes: (v << n) ^ (v >> (32-n)), logical right shift.
    #[inline(always)]
    unsafe fn rotl32<const N: i32>(v: v128) -> v128 {
        let l = i32x4_shl(v, N as u32);
        let r = u32x4_shr(v, (32 - N) as u32);
        v128_xor(l, r)
    }

    #[inline(always)]
    unsafe fn permute_badc(v: v128) -> v128 {
        // Lanes [1, 0, 3, 2] expressed as byte indices
        i8x16_shuffle::<
            4, 5, 6, 7,   0, 1, 2, 3,   12, 13, 14, 15,   8, 9, 10, 11
        >(v, v)
    }

    #[inline(always)]
    unsafe fn permute_cdab(v: v128) -> v128 {
        // Lanes [2, 3, 0, 1] expressed as byte indices
        i8x16_shuffle::<
            8, 9, 10, 11,   12, 13, 14, 15,   0, 1, 2, 3,   4, 5, 6, 7
        >(v, v)
    }

    // Unaligned vector load/store
    #[inline(always)]
    unsafe fn load_u8x16(p: *const u8) -> v128 {
        v128_load(p as *const v128)
    }
    #[inline(always)]
    unsafe fn store_u8x16(p: *mut u8, v: v128) {
        v128_store(p as *mut v128, v)
    }

    pub struct WasmSimd {
        x0: v128, x1: v128, x2: v128, x3: v128,
        x4: v128, x5: v128, x6: v128, x7: v128,
    }

    impl WasmSimd {
        #[inline(always)]
        unsafe fn rounds(&mut self) {
            let mut y0: v128;
            let mut y1: v128;
            let mut y2: v128;
            let mut y3: v128;

            for _ in 0..ROUNDS {
                // First half
                self.x4 = add32(self.x0, permute_badc(self.x4));
                self.x5 = add32(self.x1, permute_badc(self.x5));
                self.x6 = add32(self.x2, permute_badc(self.x6));
                self.x7 = add32(self.x3, permute_badc(self.x7));

                y0 = self.x2; y1 = self.x3; y2 = self.x0; y3 = self.x1;
                self.x0 = xor128(rotl32::<7>(y0), self.x4);
                self.x1 = xor128(rotl32::<7>(y1), self.x5);
                self.x2 = xor128(rotl32::<7>(y2), self.x6);
                self.x3 = xor128(rotl32::<7>(y3), self.x7);

                // Second half
                self.x4 = add32(self.x0, permute_cdab(self.x4));
                self.x5 = add32(self.x1, permute_cdab(self.x5));
                self.x6 = add32(self.x2, permute_cdab(self.x6));
                self.x7 = add32(self.x3, permute_cdab(self.x7));

                y0 = self.x1; y1 = self.x0; y2 = self.x3; y3 = self.x2;
                self.x0 = xor128(rotl32::<11>(y0), self.x4);
                self.x1 = xor128(rotl32::<11>(y1), self.x5);
                self.x2 = xor128(rotl32::<11>(y2), self.x6);
                self.x3 = xor128(rotl32::<11>(y3), self.x7);
            }
        }
    }

    impl Backend for WasmSimd {
        fn new(params: CubeHashParams) -> Self {
            let (irounds, _frounds) = rounds_for_rev(params.revision);
            unsafe {
                // Match the 128-bit baseline lane/memory layout:
                // lanes 0..3 = [outlen_bytes, BLOCKSIZE, ROUNDS, 0]
                let mut st = WasmSimd {
                    x0: u32x4(
                        (params.hash_len_bits / 8) as u32,
                        BLOCKSIZE as u32,
                        ROUNDS as u32,
                        0,
                    ),
                    x1: u32x4(0, 0, 0, 0),
                    x2: u32x4(0, 0, 0, 0),
                    x3: u32x4(0, 0, 0, 0),
                    x4: u32x4(0, 0, 0, 0),
                    x5: u32x4(0, 0, 0, 0),
                    x6: u32x4(0, 0, 0, 0),
                    x7: u32x4(0, 0, 0, 0),
                };
                for _ in 0..(irounds / ROUNDS) { st.rounds(); }
                st
            }
        }

        fn absorb_block(&mut self, block32: &[u8]) {
            debug_assert_eq!(block32.len(), BLOCKSIZE);
            unsafe {
                let p = block32.as_ptr();
                self.x0 = xor128(self.x0, load_u8x16(p));
                self.x1 = xor128(self.x1, load_u8x16(p.add(16)));
                self.rounds();
            }
        }

        fn set_finalize_flag(&mut self) {
            unsafe {
                // Place the flag in lane2 to match the 128-bit baseline layout
                self.x7 = xor128(self.x7, u32x4(0, 0, 1, 0));
            }
        }

        fn rounds_only(&mut self) { unsafe { self.rounds(); } }

        fn output_full(&self) -> [u8; 64] {
            unsafe {
                let mut out = [0u8; 64];
                store_u8x16(out.as_mut_ptr(), self.x0);
                store_u8x16(out.as_mut_ptr().add(16), self.x1);
                store_u8x16(out.as_mut_ptr().add(32), self.x2);
                store_u8x16(out.as_mut_ptr().add(48), self.x3);
                out
            }
        }
    }
}

// Re-export only when simd128 is available
#[cfg(all(target_arch = "wasm32"))]
pub use wasm32_backend::WasmSimd;