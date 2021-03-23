use crate::as_bytes::AsBytes;
use crate::consts::*;
use crate::simd::{u32x4, Vector4};
use std::io::Read;
use std::io::Stdin;

use digest::generic_array::typenum::U64;
use digest::generic_array::GenericArray;
use std::convert::TryInto;

type Output = GenericArray<u8, U64>;

fn copy(src: &[u8], dst: &mut [u8]) {
    assert!(dst.len() >= src.len());
    unsafe {
        core::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
    }
}

fn read_le_u32(input: &[u8]) -> u32 {
    let (int_bytes, _rest) = input.split_at(std::mem::size_of::<u32>());
    u32::from_le_bytes(int_bytes.try_into().unwrap())
}

fn load(data: &[u8], pos: usize) -> u32x4 {
    u32x4::new(
        read_le_u32(&data[pos + 12..pos + 16]),
        read_le_u32(&data[pos + 8..pos + 12]),
        read_le_u32(&data[pos + 4..pos + 8]),
        read_le_u32(&data[pos + 0..pos + 4]),
    )
}

pub unsafe fn _cubehash(input: &mut Stdin, irounds: i32, frounds: i32, hashlen: i32) -> Output {
    eprintln!("Hashing using CubeHash{}+16/32+{}-{}...", irounds, frounds, hashlen);
    let mut done = false;
    let mut eof = false;
    let mut more = true;
    let mut data: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize];

    let mut x0 = u32x4::new(0, ROUNDS as u32, BLOCKSIZE as u32, (hashlen / 8) as u32);
    let mut x1 = u32x4::new(0, 0, 0, 0);
    let mut x2 = u32x4::new(0, 0, 0, 0);
    let mut x3 = u32x4::new(0, 0, 0, 0);
    let mut x4 = u32x4::new(0, 0, 0, 0);
    let mut x5 = u32x4::new(0, 0, 0, 0);
    let mut x6 = u32x4::new(0, 0, 0, 0);
    let mut x7 = u32x4::new(0, 0, 0, 0);

    let mut y0: u32x4;
    let mut y1: u32x4;
    let mut y2: u32x4;
    let mut y3: u32x4;

    let mut datasize = irounds / ROUNDS * BLOCKSIZE;

    while !done {
        let mut pos = 0;
        let end = datasize - 1;

        while pos < end {
            x0 = x0 ^ load(&data, pos as usize);
            pos += 16;

            x1 = x1 ^ load(&data, pos as usize);
            pos += 16;

            for _i in 0..ROUNDS {
                x4 = x0 + x4.shuffle_1032();
                x5 = x1 + x5.shuffle_1032();
                x6 = x2 + x6.shuffle_1032();
                x7 = x3 + x7.shuffle_1032();
                y0 = x2;
                y1 = x3;
                y2 = x0;
                y3 = x1;
                x0 = y0.rotate_left_const(7) ^ y0.rotate_right_const(25);
                x1 = y1.rotate_left_const(7) ^ y1.rotate_right_const(25);
                x2 = y2.rotate_left_const(7) ^ y2.rotate_right_const(25);
                x3 = y3.rotate_left_const(7) ^ y3.rotate_right_const(25);
                x0 = x0 ^ x4;
                x1 = x1 ^ x5;
                x2 = x2 ^ x6;
                x3 = x3 ^ x7;

                x4 = x0 + x4.shuffle_2301();
                x5 = x1 + x5.shuffle_2301();
                x6 = x2 + x6.shuffle_2301();
                x7 = x3 + x7.shuffle_2301();
                y0 = x1;
                y1 = x0;
                y2 = x3;
                y3 = x2;
                x0 = y0.rotate_left_const(11) ^ y0.rotate_right_const(21);
                x1 = y1.rotate_left_const(11) ^ y1.rotate_right_const(21);
                x2 = y2.rotate_left_const(11) ^ y2.rotate_right_const(21);
                x3 = y3.rotate_left_const(11) ^ y3.rotate_right_const(21);
                x0 = x0 ^ x4;
                x1 = x1 ^ x5;
                x2 = x2 ^ x6;
                x3 = x3 ^ x7;
            }
        }
        done = !more;

        if more {
            if eof {
                datasize = frounds / ROUNDS * BLOCKSIZE;
                for i in &mut data[0..datasize as usize] {
                    *i = 0
                }
                x7 = x7 ^ u32x4::new(0, 1, 0, 0);
                more = false;
            } else {
                datasize = input.read(&mut data).unwrap() as i32;
                if datasize < BUFSIZE {
                    let padsize = BLOCKSIZE - datasize % BLOCKSIZE;
                    for i in &mut data[datasize as usize..(datasize + padsize) as usize] {
                        *i = 0
                    }
                    data[datasize as usize] = 0x80;
                    datasize += padsize;
                    eof = true;
                }
            }
        }
    }

    let mut out = GenericArray::default();
    let buf = [x0.reverse(), x1.reverse(), x2.reverse(), x3.reverse()];
    copy(buf.as_bytes(), &mut out);
    out
}

pub fn cubehash(input: &mut Stdin, revision: i32, hashlen: i32) -> Output {
    unsafe {
        match if hashlen <= MAXHASHLEN && hashlen % 8 == 0 {
            revision
        } else {
            0
        } {
            3 => return _cubehash(input, 16, 32, hashlen),
            2 => return _cubehash(input, 160, 160, hashlen),
            _ => return GenericArray::default(),
        };
    }
}
