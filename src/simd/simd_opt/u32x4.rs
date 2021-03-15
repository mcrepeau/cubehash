use crate::simd::simdty::u32x4;

#[inline(always)]
pub fn rotate_right_const(vec: u32x4, n: u32) -> u32x4 {
    vec >> u32x4::new(n, n, n, n)
}

#[inline(always)]
pub fn rotate_left_const(vec: u32x4, n: u32) -> u32x4 {
    vec << u32x4::new(n, n, n, n)
}
