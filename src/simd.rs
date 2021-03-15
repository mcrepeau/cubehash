mod simd_opt;
mod simdint;
mod simdop;
mod simdty;

pub use self::simdty::u32x4;

pub trait Vector4<T>: Copy {
    fn gather(src: &[T], i0: usize, i1: usize, i2: usize, i3: usize) -> Self;

    fn from_le(self) -> Self;
    fn to_le(self) -> Self;

    fn wrapping_add(self, rhs: Self) -> Self;

    fn rotate_right_const(self, n: u32) -> Self;
    fn rotate_left_const(self, n: u32) -> Self;

    fn shuffle_1032(self) -> Self;
    fn shuffle_2301(self) -> Self;

    fn reverse(self) -> Self;
}

macro_rules! impl_vector4 {
    ($vec:ident, $word:ident) => {
        impl Vector4<$word> for $vec {
            #[inline(always)]
            fn gather(src: &[$word], i0: usize, i1: usize, i2: usize, i3: usize) -> Self {
                $vec::new(src[i0], src[i1], src[i2], src[i3])
            }

            #[cfg(target_endian = "little")]
            #[inline(always)]
            fn from_le(self) -> Self {
                self
            }

            #[cfg(not(target_endian = "little"))]
            #[inline(always)]
            fn from_le(self) -> Self {
                $vec::new(
                    $word::from_le(self.0),
                    $word::from_le(self.1),
                    $word::from_le(self.2),
                    $word::from_le(self.3),
                )
            }

            #[cfg(target_endian = "little")]
            #[inline(always)]
            fn to_le(self) -> Self {
                self
            }

            #[cfg(not(target_endian = "little"))]
            #[inline(always)]
            fn to_le(self) -> Self {
                $vec::new(
                    self.0.to_le(),
                    self.1.to_le(),
                    self.2.to_le(),
                    self.3.to_le(),
                )
            }

            #[inline(always)]
            fn wrapping_add(self, rhs: Self) -> Self {
                self + rhs
            }

            #[inline(always)]
            fn rotate_right_const(self, n: u32) -> Self {
                simd_opt::$vec::rotate_right_const(self, n)
            }

            #[inline(always)]
            fn rotate_left_const(self, n: u32) -> Self {
                simd_opt::$vec::rotate_left_const(self, n)
            }

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shuffle_1032(self) -> Self {
                use crate::simd::simdint::simd_shuffle4;
                unsafe { simd_shuffle4(self, self, [1, 0, 3, 2]) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shuffle_1032(self) -> Self {
                $vec::new(self.1, self.0, self.3, self.2)
            }

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shuffle_2301(self) -> Self {
                use crate::simd::simdint::simd_shuffle4;
                unsafe { simd_shuffle4(self, self, [2, 3, 0, 1]) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shuffle_2301(self) -> Self {
                $vec::new(self.2, self.3, self.0, self.1)
            }

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn reverse(self) -> Self {
                use crate::simd::simdint::simd_shuffle4;
                unsafe { simd_shuffle4(self, self, [3, 2, 1, 0]) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn reverse(self) -> Self {
                $vec::new(self.3, self.2, self.1, self.0)
            }
        }
    };
}

impl_vector4!(u32x4, u32);
