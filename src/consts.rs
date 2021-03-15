#![allow(clippy::unreadable_literal)]

pub const BUFSIZE: i32 = 65536;
pub const ROUNDS: i32 = 16;
pub const BLOCKSIZE: i32 = 32;
pub const MAXHASHLEN: i32 = 512;

pub static CUBEHASH_IV: [u32; 8] = [
    0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19,
];
