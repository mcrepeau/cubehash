use wasm_bindgen::prelude::*;
use cubehash::{CubeHashBest, CubeHashParams};

/// Streaming CubeHash wrapper for JavaScript via wasm-bindgen.
///
/// Exposes `new(revision, hash_len_bits)`, `update(Uint8Array)`, `finalize() -> Uint8Array`.
#[wasm_bindgen]
pub struct WasmCubeHash {
    inner: Option<CubeHashBest>,
}

#[wasm_bindgen]
impl WasmCubeHash {
    /// Create a new hasher.
    ///
    /// - `revision`: 2 or 3
    /// - `hash_len_bits`: 8..=512, multiple of 8
    #[wasm_bindgen(constructor)]
    pub fn new(revision: i32, hash_len_bits: i32) -> WasmCubeHash {
        let params = CubeHashParams { revision, hash_len_bits };
        WasmCubeHash {
            inner: Some(CubeHashBest::new(params)),
        }
    }

    /// Absorb more data (Uint8Array in JS).
    pub fn update(&mut self, data: &[u8]) {
        if let Some(inner) = &mut self.inner {
            inner.update(data);
        }
    }

    /// Finalize and return the digest as a Uint8Array.
    /// Calling this more than once returns an empty array.
    pub fn finalize(&mut self) -> Vec<u8> {
        match self.inner.take() {
            Some(inner) => inner.finalize(),
            None => Vec::new(),
        }
    }
}

/// One-shot hashing helper for JavaScript.
///
/// - `revision`: 2 or 3
/// - `hash_len_bits`: 8..=512, multiple of 8
/// - `data`: Uint8Array in JS
#[wasm_bindgen]
pub fn cubehash_hash(revision: i32, hash_len_bits: i32, data: &[u8]) -> Vec<u8> {
    let params = CubeHashParams { revision, hash_len_bits };
    let mut h = CubeHashBest::new(params);
    h.update(data);
    h.finalize()
}
