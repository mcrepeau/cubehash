use std::io;
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
