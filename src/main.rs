use std::io;
use std::env;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
mod u32x4;
mod cubehash;
use crate::cubehash::cubehash;

fn help() {
    println!("Usage: cubehash [OPTIONS] [PARAMETERS]                                        ");
	println!("                                                                              ");
	println!("OPTIONS                                                                       ");
	println!("  -2    Use the second revision of proposed CubeHash parameters, implementing ");
	println!("        CubeHash160+16/32+160-h for hash length h.                            ");
	println!("  -3    Use the third revision of proposed CubeHash parameters, implementing  ");
	println!("        CubeHash16+16/32+32-h for hash length h. This is the default.         ");
	println!("  -l HASHLEN                                                                  ");
	println!("        Set the hash length in bits (default: 256). The hash length must be   ");
	println!("        positive, evenly divisible by 8, and not greater than 512.            ");
	println!("  -h    Show this help text and exit.                                         ");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdin = io::stdin();
    let mut hashlen = 256;
    let mut revision = 3;

    match args.len() {
        2 => {
            let cmd = &args[1];
            match &cmd[..] {
                "-2" => revision = 2,
                "-3" => revision = 3,
                "-h" => {
                    help();
                    return;
                },
                _ => { 
                    eprintln!("error: invalid command");
                    help();
                    return;
                }
            }
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };
            match &cmd[..] {
                "-l" => {
                    if number > 512 || number % 8 != 0 {
                        eprintln!("error: second argument not inferior or equal to 512 and divisible by 8");
                        return;
                    }
                    hashlen = number
                },
                _ => {
                    eprintln!("error: invalid command");
                    help();
                    return;
                },
            }
        },
        4 => {
            let cmd1 = &args[1];
            let cmd2 = &args[2];
            let num = &args[3];
            let number: i32 = match num.parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                },
            };

            match &cmd1[..] {
                "-2" => revision = 2,
                "-3" => revision = 3,
                _ => {
                    eprintln!("error: invalid command");
                    help();
                    return;
                },
            }

            match &cmd2[..] {
                "-l" => {
                    if number > 512 || number % 8 != 0 {
                        eprintln!("error: second argument not inferior or equal to 512 and divisible by 8");
                        return;
                    }
                    hashlen = number
                },
                _ => {
                    eprintln!("error: invalid command");
                    help();
                    return;
                },
            }
        },
        _ => {
        }
    }

    let result = cubehash(&mut stdin, revision, hashlen);

    for i in 0..hashlen / 8 {
        print!("{:02x}", result[i as usize]);
    }

    println!();
}
