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

```toml
[dependencies]
cubehash = "0.2"
```

Incremental API with fixed-size wrappers:

```rust
use cubehash::{CubeHash256, CubeHash384, CubeHash512};

let mut h256 = CubeHash256::new();
h256.update(b"hello");
h256.update(b" world");
let digest_32 = h256.finalize(); // [u8; 32]

let mut h384 = CubeHash384::new();
h384.update(b"data");
let digest_48 = h384.finalize(); // [u8; 48]

let mut h512 = CubeHash512::new();
h512.update(b"data");
let digest_64 = h512.finalize(); // [u8; 64]
```

Generic API with explicit parameters:

```rust
use cubehash::{CubeHash, CubeHashParams};

let mut h = CubeHash::new(CubeHashParams { revision: 3, hash_len_bits: 256 });
h.update(b"stream ");
h.update(b"data");
let digest = h.finalize(); // Vec<u8> length = hash_len_bits / 8
```

### CLI usage

Build and run the CLI:

```bash
cargo build --release
./target/release/cubehash -3 -l 256 < file
```

Options:
- `-2` / `-3`: select revision 2 or 3 (default rev3)
- `-l HASHLEN`: output length in bits (8..=512, multiple of 8)

### Benchmarks

- Run all benches with Criterion: `cargo bench`
- Bash script timing the hashing of testfiles: `./benchmark.sh`

There is also a manifest-based verification script used in CI to hash test files
and compare against expected outputs:

```bash
bash scripts/verify_manifest.sh  # reads testfiles/manifest.txt
```

### References

- Spec and background: `https://cubehash.cr.yp.to/`

### License

Licensed under MIT (`LICENSE-MIT`)
