use crate::cubehash::_cubehash as core_cubehash;

#[derive(Clone, Copy, Debug)]
pub struct CubeHashParams {
    pub revision: i32,       // 2 or 3
    pub hash_len_bits: i32,  // 8..=512 step 8
}

impl Default for CubeHashParams {
    fn default() -> Self { Self { revision: 3, hash_len_bits: 256 } }
}

pub struct CubeHash {
    params: CubeHashParams,
    buffer: Vec<u8>,
    finished: bool,
}

impl CubeHash {
    pub fn new(params: CubeHashParams) -> Self {
        Self { params, buffer: Vec::new(), finished: false }
    }

    pub fn update(&mut self, data: &[u8]) {
        assert!(!self.finished, "Cannot update after finalize");
        self.buffer.extend_from_slice(data);
    }

    pub fn finalize(mut self) -> Vec<u8> {
        self.finished = true;
        let mut cursor = std::io::Cursor::new(&self.buffer);
        unsafe {
            core_cubehash(
                &mut cursor,
                if self.params.revision == 3 { 16 } else { 160 },
                if self.params.revision == 3 { 32 } else { 160 },
                self.params.hash_len_bits,
            )
        }
    }

    pub fn reset(&mut self) {
        self.buffer.clear();
        self.finished = false;
    }
}

pub struct CubeHash256(CubeHash);
pub struct CubeHash384(CubeHash);
pub struct CubeHash512(CubeHash);

impl CubeHash256 {
    pub fn new() -> Self {
        Self(CubeHash::new(CubeHashParams { revision: 3, hash_len_bits: 256 }))
    }
    pub fn update(&mut self, data: &[u8]) { self.0.update(data); }
    pub fn finalize(self) -> [u8; 32] {
        let out = self.0.finalize();
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&out[..32]);
        arr
    }
    pub fn reset(&mut self) { self.0.reset(); }
}

impl CubeHash384 {
    pub fn new() -> Self {
        Self(CubeHash::new(CubeHashParams { revision: 3, hash_len_bits: 384 }))
    }
    pub fn update(&mut self, data: &[u8]) { self.0.update(data); }
    pub fn finalize(self) -> [u8; 48] {
        let out = self.0.finalize();
        let mut arr = [0u8; 48];
        arr.copy_from_slice(&out[..48]);
        arr
    }
    pub fn reset(&mut self) { self.0.reset(); }
}

impl CubeHash512 {
    pub fn new() -> Self {
        Self(CubeHash::new(CubeHashParams { revision: 3, hash_len_bits: 512 }))
    }
    pub fn update(&mut self, data: &[u8]) { self.0.update(data); }
    pub fn finalize(self) -> [u8; 64] {
        let out = self.0.finalize();
        let mut arr = [0u8; 64];
        arr.copy_from_slice(&out[..64]);
        arr
    }
    pub fn reset(&mut self) { self.0.reset(); }
}


