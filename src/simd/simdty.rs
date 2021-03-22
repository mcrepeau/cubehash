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
    pub struct Simd4<T>(pub T, pub T, pub T, pub T);
}

pub type u32x4 = Simd4<u32>;
pub type u64x4 = Simd4<u64>;

impl<T> Simd4<T> {
    #[inline(always)]
    pub fn new(e0: T, e1: T, e2: T, e3: T) -> Simd4<T> {
        Simd4(e0, e1, e2, e3)
    }
}

unsafe impl<T: Safe> Safe for Simd4<T> {}
