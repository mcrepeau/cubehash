#![allow(dead_code, non_camel_case_types)]

use crate::as_bytes::Safe;

#[cfg(feature = "simd")]
macro_rules! decl_simd {
    ($($decl:item)*) => {
        $(
            #[derive(Clone, Copy, Debug)]
            #[repr(simd)]
            $decl
        )*
    }
}

#[cfg(not(feature = "simd"))]
macro_rules! decl_simd {
    ($($decl:item)*) => {
        $(
            #[derive(Clone, Copy, Debug)]
            #[repr(C)]
            $decl
        )*
    }
}

decl_simd! {
    pub struct Simd2<T>(pub T, pub T);
    pub struct Simd4<T>(pub T, pub T, pub T, pub T);
    pub struct Simd8<T>(pub T, pub T, pub T, pub T,
                        pub T, pub T, pub T, pub T);
    pub struct Simd16<T>(pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T);
    pub struct Simd32<T>(pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T,
                         pub T, pub T, pub T, pub T);
}

pub type u64x2 = Simd2<u64>;

pub type u32x4 = Simd4<u32>;
pub type u64x4 = Simd4<u64>;

pub type u16x8 = Simd8<u16>;
pub type u32x8 = Simd8<u32>;

pub type u8x16 = Simd16<u8>;
pub type u16x16 = Simd16<u16>;

pub type u8x32 = Simd32<u8>;

impl<T> Simd4<T> {
    #[inline(always)]
    pub fn new(e0: T, e1: T, e2: T, e3: T) -> Simd4<T> {
        Simd4(e0, e1, e2, e3)
    }
}

unsafe impl<T: Safe> Safe for Simd2<T> {}
unsafe impl<T: Safe> Safe for Simd4<T> {}
unsafe impl<T: Safe> Safe for Simd8<T> {}
unsafe impl<T: Safe> Safe for Simd16<T> {}
unsafe impl<T: Safe> Safe for Simd32<T> {}
