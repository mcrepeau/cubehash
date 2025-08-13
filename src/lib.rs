#[cfg(any(feature = "force-scalar", not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))))]
mod u32x4;
mod cubehash;
mod state;
pub use cubehash::cubehash;
pub use state::{CubeHash, CubeHashParams, CubeHash256, CubeHash384, CubeHash512};

