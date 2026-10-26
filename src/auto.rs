#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(feature = "force-scalar")))]
pub mod auto_backend {
    use crate::{Backend, CubeHashParams};
    use crate::avx2::AVX2;
    use crate::scalar::Scalar;
    use crate::sse2::SSE2;

    /// Backend for x86/x86_64 that picks the best implementation at runtime.
    ///
    /// Selection happens once per hasher instance, based on CPU support:
    /// AVX2 when available, then SSE2, and the portable scalar backend
    /// otherwise. `std` caches the feature-detection result, so repeated
    /// construction is cheap.
    ///
    /// This is the backend behind `CubeHashBest`/`CubeHashAuto` on x86, so
    /// consumers get SIMD acceleration without having to set `RUSTFLAGS`.
    pub enum AutoBackend {
        Avx2(AVX2),
        Sse2(SSE2),
        Scalar(Scalar),
    }

    impl Backend for AutoBackend {
        fn new(params: CubeHashParams) -> Self {
            if std::is_x86_feature_detected!("avx2") {
                AutoBackend::Avx2(AVX2::new(params))
            } else if std::is_x86_feature_detected!("sse2") {
                AutoBackend::Sse2(SSE2::new(params))
            } else {
                AutoBackend::Scalar(Scalar::new(params))
            }
        }

        fn absorb_block(&mut self, block32: &[u8]) {
            match self {
                AutoBackend::Avx2(b) => b.absorb_block(block32),
                AutoBackend::Sse2(b) => b.absorb_block(block32),
                AutoBackend::Scalar(b) => b.absorb_block(block32),
            }
        }

        fn set_finalize_flag(&mut self) {
            match self {
                AutoBackend::Avx2(b) => b.set_finalize_flag(),
                AutoBackend::Sse2(b) => b.set_finalize_flag(),
                AutoBackend::Scalar(b) => b.set_finalize_flag(),
            }
        }

        fn rounds_only(&mut self) {
            match self {
                AutoBackend::Avx2(b) => b.rounds_only(),
                AutoBackend::Sse2(b) => b.rounds_only(),
                AutoBackend::Scalar(b) => b.rounds_only(),
            }
        }

        fn output_full(&self) -> [u8; 64] {
            match self {
                AutoBackend::Avx2(b) => b.output_full(),
                AutoBackend::Sse2(b) => b.output_full(),
                AutoBackend::Scalar(b) => b.output_full(),
            }
        }
    }
}

// Re-export at crate level
#[cfg(all(any(target_arch = "x86", target_arch = "x86_64"), not(feature = "force-scalar")))]
pub use auto_backend::AutoBackend;
