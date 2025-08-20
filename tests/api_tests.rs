use cubehash::CubeHashParams;
use cubehash::cubehash::{CubeHashBest, CubeHash256, CubeHash384, CubeHash512};

fn cubehash_bytes(data: &[u8], revision: i32, hash_bits: i32) -> Vec<u8> {
    let mut hasher = CubeHashBest::new((CubeHashParams { revision, hash_len_bits: hash_bits }));
    hasher.update(data);
    hasher.finalize().to_vec()
}

fn cubehash_hex(data: &[u8], revision: i32, hash_bits: i32) -> String {
    let bytes = cubehash_bytes(data, revision, hash_bits);
    bytes.iter().map(|b| format!("{:02x}", b)).collect::<String>()
}

#[test]
fn generic_invalid_params_return_panic() {
    for &(rev, bits) in &[(3, 7), (3, 520), (99, 256)] {
        let result = std::panic::catch_unwind(|| {
            cubehash_bytes(b"", rev, bits);
        });
        assert!(result.is_err(), "Expected panic for revision={}, bits={}", rev, bits);
    }
}

#[test]
fn output_length_matches_requested_bits() {
    for bits in [8, 16, 24, 32, 64, 128, 256, 384, 512] {
        let out = cubehash_bytes(b"abc", 3, bits);
        assert_eq!(out.len() as i32, bits / 8, "bits = {}", bits);
    }
}

#[test]
fn rev2_and_rev3_differ_for_same_input() {
    let data = b"The quick brown fox jumps over the lazy dog";
    let r2 = cubehash_bytes(data, 2, 256);
    let r3 = cubehash_bytes(data, 3, 256);
    assert!(!r2.is_empty() && !r3.is_empty());
    assert_ne!(r2, r3);
}

#[test]
fn generic_vs_streaming_consistency() {
    let inputs: &[&[u8]] = &[b"", b"a", b"abc", b"hello world", b"0123456789"];
    for &data in inputs {
        for &(revision, bits) in &[(3, 256), (3, 384), (3, 512), (2, 256)] {
            let expected = cubehash_bytes(data, revision, bits);
            let mut h = CubeHashBest::new(CubeHashParams { revision, hash_len_bits: bits });
            h.update(data);
            let got = h.finalize();
            assert_eq!(got, expected, "rev={}, bits={}", revision, bits);
        }
    }
}

#[test]
fn streaming_multi_update_matches_single_update() {
    let chunks: &[&[u8]] = &[b"stream ", b"data ", b"in ", b"parts"];
    let all: Vec<u8> = chunks.concat();

    for &(revision, bits) in &[(3, 256), (2, 256)] {
        let expected = cubehash_bytes(&all, revision, bits);
        let mut h = CubeHashBest::new(CubeHashParams { revision, hash_len_bits: bits });
        for &c in chunks { h.update(c); }
        let got = h.finalize();
        assert_eq!(got, expected, "rev={}, bits={}", revision, bits);
    }
}

#[test]
fn rev3_fixed_wrappers_match_generic() {
    let data = b"example";
    let revision = 3; // specify the revision explicitly

    let expected256 = cubehash_bytes(data, revision, 256);
    let mut h256 = CubeHash256::new(revision);
    h256.update(data);
    assert_eq!(h256.finalize().as_slice(), &expected256);

    let expected384 = cubehash_bytes(data, revision, 384);
    let mut h384 = CubeHash384::new(revision);
    h384.update(data);
    assert_eq!(h384.finalize().as_slice(), &expected384);

    let expected512 = cubehash_bytes(data, revision, 512);
    let mut h512 = CubeHash512::new(revision);
    h512.update(data);
    assert_eq!(h512.finalize().as_slice(), &expected512);
}


#[test]
fn rev2_known_vectors_empty_message() {
    let expected_512_hex = "4a1d00bbcfcb5a9562fb981e7f7db3350fe2658639d948b9d57452c22328bb32\
                            f468b072208450bad5ee178271408be0b16e5633ac8a1e3cf9864cfbfc8e043a";
    let expected_256_hex = "44c6de3ac6c73c391bf0906cb7482600ec06b216c7c54a2a8688a6a42676577d";

    let got_512 = cubehash_hex(b"", 2, 512);
    assert_eq!(got_512, expected_512_hex);

    let got_256 = cubehash_hex(b"", 2, 256);
    assert_eq!(got_256, expected_256_hex);
}

#[test]
fn rev2_known_vectors_quick_brown_fox() {
    let msg = b"The quick brown fox jumps over the lazy dog";

    let expected_512_hex = "bdba44a28cd16b774bdf3c9511def1a2baf39d4ef98b92c27cf5e37beb8990b7\
                            cdb6575dae1a548330780810618b8a5c351c1368904db7ebdf8857d596083a86";
    let expected_256_hex = "5151e251e348cbbfee46538651c06b138b10eeb71cf6ea6054d7ca5fec82eb79";

    let got_512 = cubehash_hex(msg, 2, 512);
    assert_eq!(got_512, expected_512_hex);

    let got_256 = cubehash_hex(msg, 2, 256);
    assert_eq!(got_256, expected_256_hex);
}
