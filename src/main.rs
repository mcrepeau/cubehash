use std::io;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
mod u32x4;
mod cubehash;
use crate::cubehash::cubehash;

fn main() {
    let mut stdin = io::stdin();
    let hashlen = 256;
    let revision = 3;

    let result = cubehash(&mut stdin, revision, hashlen);

    for i in 0..hashlen / 8 {
        print!("{:02x}", result[i as usize]);
    }

    println!();
}
