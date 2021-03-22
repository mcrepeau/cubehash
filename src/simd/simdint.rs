#![allow(dead_code)]

#[cfg(feature = "simd")]
extern "platform-intrinsic" {
    pub fn simd_add<T>(x: T, y: T) -> T;
    pub fn simd_shl<T>(x: T, y: T) -> T;
    pub fn simd_shr<T>(x: T, y: T) -> T;
    pub fn simd_xor<T>(x: T, y: T) -> T;
    pub fn simd_shuffle4<T, U>(v: T, w: T, idx: [u32; 4]) -> U;
}
