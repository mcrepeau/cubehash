#[allow(unused_macros)]
#[cfg(feature = "simd")]
macro_rules! transmute_shuffle {
    ($tmp:ident, $shuffle:ident, $vec:expr, $idx:expr) => {
        unsafe {
            use crate::simd::simdint::$shuffle;
            use crate::simd::simdty::$tmp;
            use core::mem::transmute;

            let tmp_i: $tmp = transmute($vec);
            let tmp_o: $tmp = $shuffle(tmp_i, tmp_i, $idx);
            transmute(tmp_o)
        }
    };
}

#[cfg(feature = "simd")]
pub mod u32x4;

#[cfg(not(feature = "simd"))]
macro_rules! simd_opt {
    ($vec:ident) => {
        pub mod $vec {
            use crate::simd::simdty::$vec;

            #[inline(always)]
            pub fn rotate_right_const(vec: $vec, n: u32) -> $vec {
                $vec::new(
                    vec.0.wrapping_shr(n),
                    vec.1.wrapping_shr(n),
                    vec.2.wrapping_shr(n),
                    vec.3.wrapping_shr(n),
                )
            }

            #[inline(always)]
            pub fn rotate_left_const(vec: $vec, n: u32) -> $vec {
                $vec::new(
                    vec.0.wrapping_shl(n),
                    vec.1.wrapping_shl(n),
                    vec.2.wrapping_shl(n),
                    vec.3.wrapping_shl(n),
                )
            }
        }
    };
}

#[cfg(not(feature = "simd"))]
simd_opt!(u32x4);
