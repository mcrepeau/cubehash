pub mod cubehash;
mod scalar;
mod sse2;
mod avx2;
mod neon;
pub use cubehash::new_hasher;

use std::cmp;

pub const BLOCKSIZE: usize = 32; // bytes
pub const ROUNDS: i32 = 16;
pub const MAXHASHLEN: i32 = 512;

#[derive(Clone, Copy, Debug)]
pub struct CubeHashParams {
    pub revision: i32,       // 2 or 3
    pub hash_len_bits: i32,  // 8..=512 step 8
}
impl Default for CubeHashParams {
    fn default() -> Self { Self { revision: 3, hash_len_bits: 256 } }
}

#[inline(always)]
fn rounds_for_rev(revision: i32) -> (i32, i32) {
    match revision {
        3 => (16, 32),   // irounds, frounds
        2 => (160, 160),
        _ => (0, 0),
    }
}

/// Every backend must expose the same surface (monomorphized).
pub trait Backend {
    /// Create state and perform the *initialization phase*:
    /// absorb (irounds/ROUNDS) zero blocks, i.e., run `irounds` rounds total.
    fn new(params: CubeHashParams) -> Self;

    /// XOR this 32-byte message block into the state and run `ROUNDS` rounds.
    fn absorb_block(&mut self, block32: &[u8]);

    /// Set the finalization flag (x7 ^= [0,1,0,0]).
    fn set_finalize_flag(&mut self);

    /// Run exactly one *group* of `ROUNDS` rounds without absorbing more input.
    fn rounds_only(&mut self);

    /// Extract full 128 bytes (x0..x3) as in your kernels, caller truncates.
    fn output_full(&self) -> [u8; 64]; // 4 * 16 = 64 in your code (x0..x3)
}

/// Generic streaming hasher; monomorphized per backend `B`.
pub struct CubeHash<B: Backend> {
    backend: B,
    block: [u8; BLOCKSIZE],
    used: usize,
    out_bits: i32,
    frounds: i32,
}

impl<B: Backend> CubeHash<B> {
    pub fn new(params: CubeHashParams) -> Self {
        assert!(params.hash_len_bits > 0
            && params.hash_len_bits <= MAXHASHLEN
            && params.hash_len_bits % 8 == 0
            && (params.revision == 2 || params.revision == 3));

        let (_ir, fr) = rounds_for_rev(params.revision);
        Self {
            backend: B::new(params),
            block: [0u8; BLOCKSIZE],
            used: 0,
            out_bits: params.hash_len_bits,
            frounds: fr,
        }
    }

    #[inline]
    pub fn update(&mut self, mut data: &[u8]) {
        if self.used > 0 {
            let need = BLOCKSIZE - self.used;
            let take = cmp::min(need, data.len());
            self.block[self.used..self.used + take].copy_from_slice(&data[..take]);
            self.used += take;
            data = &data[take..];
            if self.used == BLOCKSIZE {
                self.backend.absorb_block(&self.block);
                self.used = 0;
            }
        }

        while data.len() >= BLOCKSIZE {
            self.backend.absorb_block(&data[..BLOCKSIZE]);
            data = &data[BLOCKSIZE..];
        }

        if !data.is_empty() {
            self.block[..data.len()].copy_from_slice(data);
            self.used = data.len();
        }
    }

    pub fn finalize(mut self) -> Vec<u8> {
        // padding: 0x80 then zeros to block boundary
        self.block[self.used] = 0x80;
        self.block[self.used + 1..].fill(0);
        self.backend.absorb_block(&self.block);

        // set finalize flag
        self.backend.set_finalize_flag();

        // frounds total = (frounds/ROUNDS) groups
        for _ in 0..(self.frounds / ROUNDS) {
            self.backend.rounds_only();
        }

        let full = self.backend.output_full();
        let out_len = (self.out_bits / 8) as usize;
        full[..out_len].to_vec()
    }
}