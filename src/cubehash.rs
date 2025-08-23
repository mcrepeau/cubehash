// cubehash.rs
pub use crate::{CubeHash, CubeHashParams};

// ---- Backend auto-selection type alias ----
/// Auto-selected CubeHash type for the current target when AVX2 is available.
///
/// This is an alias for `CubeHash<crate::avx2::AVX2>`.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), target_feature = "avx2", not(feature = "force-scalar")))]
pub type CubeHashBest = CubeHash<crate::avx2::AVX2>;

/// Auto-selected CubeHash type for the current target on x86/x86_64 without AVX2.
///
/// This is an alias for `CubeHash<crate::sse2::SSE2>`.
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(target_feature = "avx2"), not(feature = "force-scalar")))]
pub type CubeHashBest = CubeHash<crate::sse2::SSE2>;

/// Auto-selected CubeHash type for the current target on AArch64 with NEON.
///
/// This is an alias for `CubeHash<crate::neon::NEON>`.
#[cfg(all(target_arch = "aarch64", not(feature = "force-scalar")))]
pub type CubeHashBest = CubeHash<crate::neon::NEON>;

/// Auto-selected CubeHash type for WASM32 targets with SIMD128.
///
/// This is an alias for `CubeHash<crate::wasm32::Wasm32>`.
#[cfg(all(target_arch = "wasm32"))]
pub type CubeHashBest = CubeHash<crate::wasm32::WasmSimd>;

/// Auto-selected CubeHash type for targets without SIMD support or when `force-scalar` is enabled.
///
/// This is an alias for `CubeHash<crate::scalar::Scalar>`.
#[cfg(any(feature = "force-scalar", not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64", target_arch = "wasm32"))))]
pub type CubeHashBest = CubeHash<crate::scalar::Scalar>;

/// Synonym for `CubeHashBest`.
///
/// Prefer this name when you want to make it explicit that the backend is chosen automatically
/// for the current platform.
///
/// Example:
/// ```rust
/// use cubehash::{CubeHashAuto, CubeHashParams};
///
/// let mut h: CubeHashAuto = CubeHashAuto::new(CubeHashParams { revision: 3, hash_len_bits: 256 });
/// h.update(b"hello");
/// let out = h.finalize();
/// assert_eq!(out.len(), 32);
/// ```
pub type CubeHashAuto = CubeHashBest;

// ------------------- Generic wrapper for N-byte hashes -------------------

/// A convenience wrapper that fixes the output size at compile time.
///
/// `CubeHashN<N>` builds on top of the auto-selected backend and exposes the same
/// `update`/`finalize` streaming API. `finalize` returns a fixed-size array of `N` bytes.
///
/// Typically you will use the aliases `CubeHash256`, `CubeHash384`, or `CubeHash512` instead of
/// instantiating this type directly.
///
/// Example:
/// ```rust
/// use cubehash::CubeHash256;
///
/// let mut h = CubeHash256::new(3); // revision 3
/// h.update(b"hello");
/// let out = h.finalize();
/// assert_eq!(out.len(), 32);
/// ```
pub struct CubeHashN<const N: usize> {
    inner: CubeHashBest,
}

impl<const N: usize> CubeHashN<N> {
    /// Construct a new fixed-size CubeHash hasher for a given revision.
    ///
    /// Arguments:
    /// - `revision`: 2 or 3
    ///
    /// The digest size is determined by `N` (in bytes).
    ///
    /// Example:
    /// ```rust
    /// use cubehash::CubeHash384;
    ///
    /// let mut h = CubeHash384::new(3);
    /// h.update(b"abc");
    /// let out = h.finalize();
    /// assert_eq!(out.len(), 48);
    /// ```
    pub fn new(revision: i32) -> Self {
        Self {
            inner: CubeHashBest::new(CubeHashParams {
                revision,
                hash_len_bits: (N * 8) as i32,
            }),
        }
    }

    /// Absorb more input into the hash state.
    ///
    /// You can call `update` any number of times with arbitrary chunk sizes.
    /// After the last call to `update`, call `finalize` to get the digest.
    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    /// Finalize the hash and return a fixed-size digest `[u8; N]`.
    ///
    /// Consumes the hasher. After calling this, the instance cannot be used again.
    pub fn finalize(self) -> [u8; N] {
        let out = self.inner.finalize();
        let mut a = [0u8; N];
        a.copy_from_slice(&out[..N]);
        a
    }
}

// ------------------- Type aliases for common hash sizes -------------------
/// Convenience alias for a 256-bit CubeHash (32-byte output).
///
/// Example:
/// ```rust
/// use cubehash::CubeHash256;
///
/// let mut h = CubeHash256::new(3);
/// h.update(b"example");
/// let out = h.finalize();
/// assert_eq!(out.len(), 32);
/// ```
pub type CubeHash256 = CubeHashN<32>;

/// Convenience alias for a 384-bit CubeHash (48-byte output).
///
/// Example:
/// ```rust
/// use cubehash::CubeHash384;
///
/// let mut h = CubeHash384::new(3);
/// h.update(b"example");
/// let out = h.finalize();
/// assert_eq!(out.len(), 48);
/// ```
pub type CubeHash384 = CubeHashN<48>;

/// Convenience alias for a 512-bit CubeHash (64-byte output).
///
/// Example:
/// ```rust
/// use cubehash::CubeHash512;
///
/// let mut h = CubeHash512::new(2);
/// h.update(b"example");
/// let out = h.finalize();
/// assert_eq!(out.len(), 64);
/// ```
pub type CubeHash512 = CubeHashN<64>;
