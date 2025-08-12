#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
mod u32x4;
mod cubehash;
pub use cubehash::cubehash;

