// cubehash.rs
pub use crate::{CubeHash, CubeHashParams};

// ---- Backend auto-selection type alias ----
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2", not(feature = "force-scalar")))]
pub type CubeHashBest = CubeHash<crate::avx2::AVX2>;
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(target_feature = "avx2"), not(feature = "force-scalar")))]
pub type CubeHashBest = CubeHash<crate::sse2::SSE2>;
#[cfg(all(target_arch = "aarch64", not(feature = "force-scalar")))]
pub type CubeHashBest = CubeHash<crate::neon::NEON>;
#[cfg(any(feature = "force-scalar", not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))))]
pub type CubeHashBest = CubeHash<crate::scalar::Scalar>;
pub type CubeHashAuto = CubeHashBest;

// ------------------- Generic wrapper for N-byte hashes -------------------
pub struct CubeHashN<const N: usize> {
    inner: CubeHashBest,
}

impl<const N: usize> CubeHashN<N> {
    /// Create a new CubeHash instance with the given revision
    pub fn new(revision: i32) -> Self {
        Self {
            inner: CubeHashBest::new(CubeHashParams {
                revision,
                hash_len_bits: (N * 8) as i32,
            }),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn finalize(self) -> [u8; N] {
        let out = self.inner.finalize();
        let mut a = [0u8; N];
        a.copy_from_slice(&out[..N]);
        a
    }
}

// ------------------- Type aliases for common hash sizes -------------------
pub type CubeHash256 = CubeHashN<32>;
pub type CubeHash384 = CubeHashN<48>;
pub type CubeHash512 = CubeHashN<64>;
