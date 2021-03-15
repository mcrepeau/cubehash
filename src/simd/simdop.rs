#[cfg(feature = "simd")]
use crate::simd::simdint;
use crate::simd::simdty::u32x4;

use core::ops::{Add, BitXor, Shl, Shr};

macro_rules! impl_ops {
    ($vec:ident) => {
        impl Add for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                unsafe { simdint::simd_add(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn add(self, rhs: Self) -> Self::Output {
                $vec::new(
                    self.0.wrapping_add(rhs.0),
                    self.1.wrapping_add(rhs.1),
                    self.2.wrapping_add(rhs.2),
                    self.3.wrapping_add(rhs.3),
                )
            }
        }

        impl BitXor for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                unsafe { simdint::simd_xor(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn bitxor(self, rhs: Self) -> Self::Output {
                $vec::new(
                    self.0 ^ rhs.0,
                    self.1 ^ rhs.1,
                    self.2 ^ rhs.2,
                    self.3 ^ rhs.3,
                )
            }
        }

        impl Shl<$vec> for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shl(self, rhs: Self) -> Self::Output {
                unsafe { simdint::simd_shl(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shl(self, rhs: Self) -> Self::Output {
                $vec::new(
                    self.0 << rhs.0,
                    self.1 << rhs.1,
                    self.2 << rhs.2,
                    self.3 << rhs.3,
                )
            }
        }

        impl Shr<$vec> for $vec {
            type Output = Self;

            #[cfg(feature = "simd")]
            #[inline(always)]
            fn shr(self, rhs: Self) -> Self::Output {
                unsafe { simdint::simd_shr(self, rhs) }
            }

            #[cfg(not(feature = "simd"))]
            #[inline(always)]
            fn shr(self, rhs: Self) -> Self::Output {
                $vec::new(
                    self.0 >> rhs.0,
                    self.1 >> rhs.1,
                    self.2 >> rhs.2,
                    self.3 >> rhs.3,
                )
            }
        }
    };
}

impl_ops!(u32x4);
