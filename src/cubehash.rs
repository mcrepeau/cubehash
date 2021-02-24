use std::io::Read;
use std::io::Stdin;
#[cfg(target_arch = "x86")]
use core::arch::x86::__m128i;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::__m128i;
#[cfg(target_arch = "x86")]
use core::arch::x86::_mm_set_epi32;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_set_epi32;
#[cfg(target_arch = "x86")]
use core::arch::x86::_mm_xor_si128;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_xor_si128;
#[cfg(target_arch = "x86")]
use core::arch::x86::_mm_loadu_si128;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_loadu_si128;
#[cfg(target_arch = "x86")]
use core::arch::x86::_mm_add_epi32;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_add_epi32;
#[cfg(target_arch = "x86")]
use core::arch::x86::_mm_shuffle_epi32;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_shuffle_epi32;
#[cfg(target_arch = "x86")]
use core::arch::x86::_mm_slli_epi32;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_slli_epi32;
#[cfg(target_arch = "x86")]
use core::arch::x86::_mm_srli_epi32;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::_mm_srli_epi32;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
use crate::u32x4::xor;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
use crate::u32x4::add;
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
use crate::u32x4::U32x4;

const BUFSIZE: i32 = 65536;
const ROUNDS: i32 = 16;
const BLOCKSIZE: i32 = 32;
const MAXHASHLEN: i32 = 512;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub unsafe fn _cubehash(input: &mut Stdin, irounds: i32, frounds: i32, hashlen: i32) -> Vec<u8> {
    //eprintln!("Hashing using CubeHash{}+16/32+{}-{}...", irounds, frounds, hashlen);
    
    let mut done = false;
    let mut eof = false;
    let mut more = true;
    let mut data: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize];

    let mut x0: __m128i = _mm_set_epi32(0, ROUNDS, BLOCKSIZE, hashlen / 8);
    let mut x1: __m128i = _mm_set_epi32(0, 0, 0, 0);
    let mut x2: __m128i = _mm_set_epi32(0, 0, 0, 0);
    let mut x3: __m128i = _mm_set_epi32(0, 0, 0, 0);
    let mut x4: __m128i = _mm_set_epi32(0, 0, 0, 0);
    let mut x5: __m128i = _mm_set_epi32(0, 0, 0, 0);
    let mut x6: __m128i = _mm_set_epi32(0, 0, 0, 0);
    let mut x7: __m128i = _mm_set_epi32(0, 0, 0, 0);
    
    let mut y0: __m128i;
    let mut y1: __m128i;
    let mut y2: __m128i;
    let mut y3: __m128i;

    let mut datasize = irounds / ROUNDS * BLOCKSIZE;

    while !done {
        let mut pos: *const __m128i = &data[0 as usize] as *const _ as *const __m128i;
        let end: *const __m128i = &data[(datasize - 1) as usize] as *const _ as *const __m128i;

        while pos < end {
            x0 = _mm_xor_si128(x0, _mm_loadu_si128(pos));
            pos = pos.add(1);

            x1 = _mm_xor_si128(x1, _mm_loadu_si128(pos));
            pos = pos.add(1);
            
            for _i in 0..ROUNDS {
                x4 = _mm_add_epi32(x0, _mm_shuffle_epi32(x4, 0xb1));
                x5 = _mm_add_epi32(x1, _mm_shuffle_epi32(x5, 0xb1));
                x6 = _mm_add_epi32(x2, _mm_shuffle_epi32(x6, 0xb1));
                x7 = _mm_add_epi32(x3, _mm_shuffle_epi32(x7, 0xb1));
                y0 = x2;
                y1 = x3;
                y2 = x0;
                y3 = x1;
                x0 = _mm_xor_si128(_mm_slli_epi32(y0, 7), _mm_srli_epi32(y0, 25));
                x1 = _mm_xor_si128(_mm_slli_epi32(y1, 7), _mm_srli_epi32(y1, 25));
                x2 = _mm_xor_si128(_mm_slli_epi32(y2, 7), _mm_srli_epi32(y2, 25));
                x3 = _mm_xor_si128(_mm_slli_epi32(y3, 7), _mm_srli_epi32(y3, 25));
                x0 = _mm_xor_si128(x0, x4);
                x1 = _mm_xor_si128(x1, x5);
                x2 = _mm_xor_si128(x2, x6);
                x3 = _mm_xor_si128(x3, x7);

                x4 = _mm_add_epi32(x0, _mm_shuffle_epi32(x4, 0x4e));
                x5 = _mm_add_epi32(x1, _mm_shuffle_epi32(x5, 0x4e));
                x6 = _mm_add_epi32(x2, _mm_shuffle_epi32(x6, 0x4e));
                x7 = _mm_add_epi32(x3, _mm_shuffle_epi32(x7, 0x4e));
                y0 = x1;
                y1 = x0;
                y2 = x3;
                y3 = x2;
                x0 = _mm_xor_si128(_mm_slli_epi32(y0, 11), _mm_srli_epi32(y0, 21));
                x1 = _mm_xor_si128(_mm_slli_epi32(y1, 11), _mm_srli_epi32(y1, 21));
                x2 = _mm_xor_si128(_mm_slli_epi32(y2, 11), _mm_srli_epi32(y2, 21));
                x3 = _mm_xor_si128(_mm_slli_epi32(y3, 11), _mm_srli_epi32(y3, 21));
                x0 = _mm_xor_si128(x0, x4);
                x1 = _mm_xor_si128(x1, x5);
                x2 = _mm_xor_si128(x2, x6);
                x3 = _mm_xor_si128(x3, x7);
            }
        }
        done = !more;

        if more {
            if eof {
                datasize = frounds / ROUNDS * BLOCKSIZE;
                for i in &mut data[0..datasize as usize] { *i = 0 }
                x7 = _mm_xor_si128(x7, _mm_set_epi32(0, 1, 0, 0));
                more = false;
            } else {
                datasize = input.read(&mut data).unwrap() as i32;
                if datasize < BUFSIZE {
                    let padsize = BLOCKSIZE - datasize % BLOCKSIZE;
                    for i in &mut data[datasize as usize..(datasize + padsize) as usize] { *i = 0 }
                    data[datasize as usize] = 0x80;
                    datasize += padsize;
                    eof = true;
                }
            }
        }
    }
    
    let x0_buffer = std::mem::transmute::<__m128i, [u8; 16]>(x0);
    let x1_buffer = std::mem::transmute::<__m128i, [u8; 16]>(x1);
    let x2_buffer = std::mem::transmute::<__m128i, [u8; 16]>(x2);
    let x3_buffer = std::mem::transmute::<__m128i, [u8; 16]>(x3);

    return [x0_buffer, x1_buffer, x2_buffer, x3_buffer].concat()
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub unsafe fn _cubehash(input: &mut Stdin, irounds: i32, frounds: i32, hashlen: i32) -> Vec<u8> {
    //eprintln!("Hashing using CubeHash{}+16/32+{}-{}...", irounds, frounds, hashlen);
    
    let mut done = false;
    let mut eof = false;
    let mut more = true;
    let mut data: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize];

    let mut x0 = U32x4 { a: 0, b: ROUNDS as u32, c: BLOCKSIZE as u32, d: (hashlen / 8) as u32 };
    let mut x1 = U32x4 { a: 0, b: 0, c: 0, d: 0 };
    let mut x2 = U32x4 { a: 0, b: 0, c: 0, d: 0 };
    let mut x3 = U32x4 { a: 0, b: 0, c: 0, d: 0 };
    let mut x4 = U32x4 { a: 0, b: 0, c: 0, d: 0 };
    let mut x5 = U32x4 { a: 0, b: 0, c: 0, d: 0 };
    let mut x6 = U32x4 { a: 0, b: 0, c: 0, d: 0 };
    let mut x7 = U32x4 { a: 0, b: 0, c: 0, d: 0 };

    let mut y0: U32x4;
    let mut y1: U32x4;
    let mut y2: U32x4;
    let mut y3: U32x4;

    let mut datasize = irounds / ROUNDS * BLOCKSIZE;

    while !done {
        let mut pos = 0;
        let end = datasize - 1;

        while pos < end {
            x0 = xor(x0, U32x4::load(&data[pos as usize..(pos + 16) as usize]));
            pos += 16;

            x1 = xor(x1, U32x4::load(&data[pos as usize..(pos + 16) as usize]));
            pos += 16;
            
            for _i in 0..ROUNDS {
                x4 = add(x0, x4.permute_badc());
                x5 = add(x1, x5.permute_badc());
                x6 = add(x2, x6.permute_badc());
                x7 = add(x3, x7.permute_badc());
                y0 = x2;
                y1 = x3;
                y2 = x0;
                y3 = x1;
                x0 = xor(y0.shift_left(7), y0.shift_right(25));
                x1 = xor(y1.shift_left(7), y1.shift_right(25));
                x2 = xor(y2.shift_left(7), y2.shift_right(25));
                x3 = xor(y3.shift_left(7), y3.shift_right(25));
                x0 = xor(x0, x4);
                x1 = xor(x1, x5);
                x2 = xor(x2, x6);
                x3 = xor(x3, x7);

                x4 = add(x0, x4.permute_cdab());
                x5 = add(x1, x5.permute_cdab());
                x6 = add(x2, x6.permute_cdab());
                x7 = add(x3, x7.permute_cdab());
                y0 = x1;
                y1 = x0;
                y2 = x3;
                y3 = x2;
                x0 = xor(y0.shift_left(11), y0.shift_right(21));
                x1 = xor(y1.shift_left(11), y1.shift_right(21));
                x2 = xor(y2.shift_left(11), y2.shift_right(21));
                x3 = xor(y3.shift_left(11), y3.shift_right(21));
                x0 = xor(x0, x4);
                x1 = xor(x1, x5);
                x2 = xor(x2, x6);
                x3 = xor(x3, x7);
            }
        }
        done = !more;

        if more {
            if eof {
                datasize = frounds / ROUNDS * BLOCKSIZE;
                for i in &mut data[0..datasize as usize] { *i = 0 }
                x7 = xor(x7, U32x4 { a: 0, b: 1, c: 0, d: 0 });
                more = false;
            } else {
                datasize = input.read(&mut data).unwrap() as i32;
                if datasize < BUFSIZE {
                    let padsize = BLOCKSIZE - datasize % BLOCKSIZE;
                    for i in &mut data[datasize as usize..(datasize + padsize) as usize] { *i = 0 }
                    data[datasize as usize] = 0x80;
                    datasize += padsize;
                    eof = true;
                }
            }
        }
    }

    return [x0.transmute(), x1.transmute(), x2.transmute(), x3.transmute()].concat()
}

pub fn cubehash(input: &mut Stdin, revision: i32, hashlen: i32) -> Vec<u8> {
    unsafe {
        match if hashlen <= MAXHASHLEN && hashlen % 8 == 0 { revision }  else  { 0 } {
            3 => return _cubehash(input, 16, 32, hashlen),
            2 => return _cubehash(input, 160, 160, hashlen),
            _ => return Vec::new(),
        };
    }
}