# CubeHash

This is a Rust implementation of the CubeHash Hash function both using SIMD intrinsics for x86 and native Rust. It is based on the C99 implementation written here: <https://github.com/DennisMitchell/cubehash> 

## How to use:

### Compile
` cargo build --release`
### Compile with platform-intrinsics
`cargo +nightly build --features="simd, simd_opt, simd_asm" --release
### Run
` ./target/release/cubehash < file`
### More info
<https://cubehash.cr.yp.to/>
