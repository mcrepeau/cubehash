use std::io::{self, Read};
use std::env;
use cubehash::{new_hasher, CubeHashParams, BLOCKSIZE};

fn help() {
    println!("Usage: cubehash [OPTIONS] [STRING]");
    println!();
    println!("OPTIONS");
    println!("  -2        Use revision 2 (CubeHash160+16/32+h)");
    println!("  -3        Use revision 3 (CubeHash16+16/32+h, default)");
    println!("  -l HASHLEN  Set hash length in bits (default: 256, ≤512, divisible by 8)");
    println!("  -h        Show this help text and exit");
    println!();
    println!("If STRING is provided, it is hashed directly. Otherwise, input is read from stdin.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut hashlen = 256;
    let mut revision = 3;
    let mut string_input: Option<String> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-2" => revision = 2,
            "-3" => revision = 3,
            "-l" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("error: -l requires a number");
                    help();
                    return;
                }
                let n: i32 = match args[i].parse() {
                    Ok(x) => x,
                    Err(_) => {
                        eprintln!("error: invalid hash length");
                        help();
                        return;
                    }
                };
                if n > 512 || n % 8 != 0 {
                    eprintln!("error: hash length must be ≤ 512 and divisible by 8");
                    return;
                }
                hashlen = n;
            }
            "-h" => { help(); return; }
            s => {
                // Any other argument is treated as string input
                string_input = Some(s.to_string());
            }
        }
        i += 1;
    }

    let params = CubeHashParams { revision, hash_len_bits: hashlen };
    let mut hasher = new_hasher(params);

    if let Some(input) = string_input {
        // Hash the provided string directly
        hasher.update(input.as_bytes());
    } else {
        // Stream from stdin in BLOCKSIZE chunks
        let mut buffer = [0u8; BLOCKSIZE];
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        loop {
            let n = handle.read(&mut buffer).expect("Failed to read stdin");
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }
    }

    let result = hasher.finalize();
    for byte in result {
        print!("{:02x}", byte);
    }
    println!();
}
