//! Digest verification for the wasm32 backends, run under a WASI runtime
//! (e.g. `wasmtime`). Built twice by CI:
//!
//! - with `RUSTFLAGS="-C target-feature=+simd128"` -> SIMD128 backend
//! - without -> portable scalar backend
//!
//! Exits 0 when every vector matches, 1 otherwise, so the exit code is all
//! CI needs to check.
//!
//! Vector provenance: rev2 values for the empty string and "The quick brown
//! fox..." are the official CubeHash160+16/32+160 digests (also asserted in
//! tests/api_tests.rs). The remaining values were produced by the scalar
//! implementation, which those official vectors anchor, so this binary is a
//! genuine cross-check of the wasm backends rather than a self-comparison.

use cubehash::{CubeHashBest, CubeHashParams};

/// (input, revision, hash_bits, expected_hex)
const VECTORS: &[(&str, i32, i32, &str)] = &[
    (
        "",
        2,
        512,
        "4a1d00bbcfcb5a9562fb981e7f7db3350fe2658639d948b9d57452c22328bb32\
         f468b072208450bad5ee178271408be0b16e5633ac8a1e3cf9864cfbfc8e043a",
    ),
    (
        "",
        2,
        256,
        "44c6de3ac6c73c391bf0906cb7482600ec06b216c7c54a2a8688a6a42676577d",
    ),
    (
        "",
        3,
        256,
        "67dfa7b6b3cb27c58c19db1d7bbb7c4596913e25f228ddfb9910ddf3c5cad2eb",
    ),
    (
        "",
        3,
        512,
        "37045cca405ee6fbdf815ed8b57c971bb78dafb58f3ef676c977a716f66dbd8f\
         376fef59d2e0687cf5608c5dad53ba42c8456269f3f3bcfb27d9b75caaa26e11",
    ),
    (
        "The quick brown fox jumps over the lazy dog",
        2,
        256,
        "5151e251e348cbbfee46538651c06b138b10eeb71cf6ea6054d7ca5fec82eb79",
    ),
    (
        "The quick brown fox jumps over the lazy dog",
        2,
        512,
        "bdba44a28cd16b774bdf3c9511def1a2baf39d4ef98b92c27cf5e37beb8990b7\
         cdb6575dae1a548330780810618b8a5c351c1368904db7ebdf8857d596083a86",
    ),
    (
        "The quick brown fox jumps over the lazy dog",
        3,
        512,
        "a9ba7b8c6b4ecc6660bb3b35f076db7fce4930296491922744c67ef08dc1217c\
         e5eb26bb25247e3bc8904b46d468455e6807c21410c1fb95e44824dc7d57c7ff",
    ),
    (
        // 39 bytes: spans two 32-byte blocks
        "0123456789abcdef0123456789abcdef0123456",
        2,
        256,
        "ec82c7155b9b299fcde2463132a9b5404e292f29a93638216434ee52c5904ca1",
    ),
    (
        "0123456789abcdef0123456789abcdef0123456",
        3,
        512,
        "456d7e7822917bb0189c26ed74f08ae3005a3bfd7f5cc3830354df80a85793f9\
         d8bc8dbe9e3d14d3961e9fb44250cccc47cab627cea1eab8c0cd21d805571bd7",
    ),
];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Hash `data` through several update() chunkings and require a consistent result.
fn digest_all_chunkings(data: &[u8], revision: i32, bits: i32) -> Vec<u8> {
    let params = CubeHashParams { revision, hash_len_bits: bits };

    let mut h = CubeHashBest::new(params);
    h.update(data);
    let expected = h.finalize();

    for chunk_size in [1usize, 7, 31, 32, 33] {
        let mut h = CubeHashBest::new(params);
        for chunk in data.chunks(chunk_size) {
            h.update(chunk);
        }
        assert_eq!(h.finalize(), expected, "chunk_size={chunk_size}");
    }
    expected
}

fn main() {
    let backend = if cfg!(target_feature = "simd128") {
        "wasm32-simd128"
    } else {
        "scalar"
    };

    let mut failures = 0;
    for (input, revision, bits, expected) in VECTORS {
        let got = digest_all_chunkings(input.as_bytes(), *revision, *bits);
        let got_hex = hex(&got);
        if got_hex == *expected {
            println!("OK: rev={revision} bits={bits} input_len={} ({backend})", input.len());
        } else {
            eprintln!(
                "MISMATCH: rev={revision} bits={bits} input_len={} ({backend})\n  expected: {expected}\n  actual:   {got_hex}",
                input.len()
            );
            failures += 1;
        }
    }

    if failures > 0 {
        eprintln!("{failures} digest check(s) failed ({backend})");
        std::process::exit(1);
    }
    println!("all digest checks passed ({backend})");
}
