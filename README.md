## CubeHash

Rust implementation of the CubeHash hash function (revisions 2 and 3) with SIMD
acceleration and a portable scalar fallback. Based on the C99 implementation by
Dennis Mitchell (`https://github.com/DennisMitchell/cubehash`).

### Features and platform support

- **x86/x86_64**: SSE2 and AVX2 intrinsics
- **AArch64**: NEON intrinsics
- **Portable scalar**: always available; can be forced with the `force-scalar` feature
- Both **CubeHash rev2** and **rev3** are supported; CLI allows `-2`/`-3` selection

### Library usage

Add to your `Cargo.toml`:
```
[dependencies]
cubehash = "0.3"
# optionally force the portable (non-SIMD) backend:
# cubehash = { version = "0.3", features = ["force-scalar"] }
```
Incremental API with fixed-size wrappers (choose revision 2 or 3):
```
use cubehash::{CubeHash256, CubeHash384, CubeHash512};

let mut h256 = CubeHash256::new(3); // revision 3
h256.update(b"hello");
h256.update(b" world");
let digest_32: [u8; 32] = h256.finalize();

let mut h384 = CubeHash384::new(3);
h384.update(b"data");
let digest_48: [u8; 48] = h384.finalize();

let mut h512 = CubeHash512::new(2); // revision 2
h512.update(b"data");
let digest_64: [u8; 64] = h512.finalize();
```
Generic streaming API with explicit parameters (auto-selects the best backend):
```
rust
use cubehash::{CubeHashParams};
use cubehash::cubehash::{CubeHashBest}

let mut h = CubeHashBest::new(CubeHashParams { revision: 3, hash_len_bits: 256 });
h.update(b"stream ");
h.update(b"data");
let digest: Vec<u8> = h.finalize(); // length = hash_len_bits / 8
```

### CLI usage

Build and run the CLI:
```
cargo build --release
./target/release/cubehash -3 -l 256 < file
```
Hash a string directly:
```
./target/release/cubehash -3 -l 256 "hello world"
```
Or via stdin:
```
echo -n "hello world" | ./target/release/cubehash -3 -l 256
```
Options:
- `-2` / `-3`: select revision 2 or 3 (default rev3)
- `-l HASHLEN`: output length in bits (8..=512, multiple of 8)
- `-h`: show help

### Benchmarks

- Run all benches with Criterion: `cargo bench`
- Bash script timing the hashing of testfiles: `./benchmark.sh`

There is also a manifest-based verification script used in CI to hash test files
and compare against expected outputs:
`./scripts/verify_manifest.sh`

### References

- Spec and background: `https://cubehash.cr.yp.to/`

### License

Licensed under MIT (`LICENSE-MIT`)
