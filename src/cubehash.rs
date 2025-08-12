use std::io::Read;
#[cfg(all(target_arch = "x86", target_feature = "sse2"))]
use core::arch::x86::{
    __m128i, _mm_set_epi32, _mm_xor_si128, _mm_loadu_si128, _mm_add_epi32, _mm_shuffle_epi32, _mm_slli_epi32, _mm_srli_epi32
};
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::{
    __m128i, _mm_set_epi32, _mm_xor_si128, _mm_loadu_si128, _mm_add_epi32, _mm_shuffle_epi32, _mm_slli_epi32, _mm_srli_epi32
};
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::{
    uint32x4_t, vaddq_u32, vdupq_n_u32, vextq_u32, vld1q_u32, vrev64q_u32, vshlq_n_u32, vshrq_n_u32,
    veorq_u32, vst1q_u32,
};
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
use crate::u32x4::{add, xor, U32x4};

const BUFSIZE: i32 = 65536;
const ROUNDS: i32 = 16;
const BLOCKSIZE: i32 = 32;
const MAXHASHLEN: i32 = 512;

#[cfg(any(all(target_arch = "x86", target_feature = "sse2"), target_arch = "x86_64"))]
pub unsafe fn _cubehash<R: Read>(input: &mut R, irounds: i32, frounds: i32, hashlen: i32) -> Vec<u8> {
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

#[cfg(target_arch = "aarch64")]
pub unsafe fn _cubehash<R: Read>(input: &mut R, irounds: i32, frounds: i32, hashlen: i32) -> Vec<u8> {
    let mut done = false;
    let mut eof = false;
    let mut more = true;
    let mut data: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize];

    let mut x0: uint32x4_t = vld1q_u32([0u32, ROUNDS as u32, BLOCKSIZE as u32, (hashlen / 8) as u32].as_ptr());
    let mut x1: uint32x4_t = vdupq_n_u32(0);
    let mut x2: uint32x4_t = vdupq_n_u32(0);
    let mut x3: uint32x4_t = vdupq_n_u32(0);
    let mut x4: uint32x4_t = vdupq_n_u32(0);
    let mut x5: uint32x4_t = vdupq_n_u32(0);
    let mut x6: uint32x4_t = vdupq_n_u32(0);
    let mut x7: uint32x4_t = vdupq_n_u32(0);

    let mut y0: uint32x4_t;
    let mut y1: uint32x4_t;
    let mut y2: uint32x4_t;
    let mut y3: uint32x4_t;

    let mut datasize = irounds / ROUNDS * BLOCKSIZE;

    #[inline(always)]
    unsafe fn load_le_u32x4_reversed(bytes: &[u8]) -> uint32x4_t {
        let a = u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]);
        let b = u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]);
        let c = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
        let d = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        vld1q_u32([a, b, c, d].as_ptr())
    }

    #[inline(always)]
    unsafe fn to_bytes_d_c_b_a(v: uint32x4_t) -> [u8; 16] {
        let mut arr = [0u32; 4];
        vst1q_u32(arr.as_mut_ptr(), v);
        let bytes = [
            arr[3].to_le_bytes(),
            arr[2].to_le_bytes(),
            arr[1].to_le_bytes(),
            arr[0].to_le_bytes(),
        ];
        [
            bytes[0][0], bytes[0][1], bytes[0][2], bytes[0][3],
            bytes[1][0], bytes[1][1], bytes[1][2], bytes[1][3],
            bytes[2][0], bytes[2][1], bytes[2][2], bytes[2][3],
            bytes[3][0], bytes[3][1], bytes[3][2], bytes[3][3],
        ]
    }

    #[inline(always)]
    unsafe fn permute_badc(v: uint32x4_t) -> uint32x4_t { vrev64q_u32(v) }
    #[inline(always)]
    unsafe fn permute_cdab(v: uint32x4_t) -> uint32x4_t { vextq_u32::<2>(v, v) }

    while !done {
        let mut pos: i32 = 0;
        let end: i32 = datasize - 1;

        while pos < end {
            x0 = veorq_u32(x0, load_le_u32x4_reversed(&data[pos as usize..(pos + 16) as usize]));
            pos += 16;

            x1 = veorq_u32(x1, load_le_u32x4_reversed(&data[pos as usize..(pos + 16) as usize]));
            pos += 16;
            
            for _i in 0..ROUNDS {
                x4 = vaddq_u32(x0, permute_badc(x4));
                x5 = vaddq_u32(x1, permute_badc(x5));
                x6 = vaddq_u32(x2, permute_badc(x6));
                x7 = vaddq_u32(x3, permute_badc(x7));
                y0 = x2;
                y1 = x3;
                y2 = x0;
                y3 = x1;
                x0 = veorq_u32(vshlq_n_u32::<7>(y0), vshrq_n_u32::<25>(y0));
                x1 = veorq_u32(vshlq_n_u32::<7>(y1), vshrq_n_u32::<25>(y1));
                x2 = veorq_u32(vshlq_n_u32::<7>(y2), vshrq_n_u32::<25>(y2));
                x3 = veorq_u32(vshlq_n_u32::<7>(y3), vshrq_n_u32::<25>(y3));
                x0 = veorq_u32(x0, x4);
                x1 = veorq_u32(x1, x5);
                x2 = veorq_u32(x2, x6);
                x3 = veorq_u32(x3, x7);

                x4 = vaddq_u32(x0, permute_cdab(x4));
                x5 = vaddq_u32(x1, permute_cdab(x5));
                x6 = vaddq_u32(x2, permute_cdab(x6));
                x7 = vaddq_u32(x3, permute_cdab(x7));
                y0 = x1;
                y1 = x0;
                y2 = x3;
                y3 = x2;
                x0 = veorq_u32(vshlq_n_u32::<11>(y0), vshrq_n_u32::<21>(y0));
                x1 = veorq_u32(vshlq_n_u32::<11>(y1), vshrq_n_u32::<21>(y1));
                x2 = veorq_u32(vshlq_n_u32::<11>(y2), vshrq_n_u32::<21>(y2));
                x3 = veorq_u32(vshlq_n_u32::<11>(y3), vshrq_n_u32::<21>(y3));
                x0 = veorq_u32(x0, x4);
                x1 = veorq_u32(x1, x5);
                x2 = veorq_u32(x2, x6);
                x3 = veorq_u32(x3, x7);
            }
        }
        done = !more;

        if more {
            if eof {
                datasize = frounds / ROUNDS * BLOCKSIZE;
                for i in &mut data[0..datasize as usize] { *i = 0 }
                x7 = veorq_u32(x7, vld1q_u32([0u32, 1, 0, 0].as_ptr()));
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

    let out = [
        to_bytes_d_c_b_a(x0),
        to_bytes_d_c_b_a(x1),
        to_bytes_d_c_b_a(x2),
        to_bytes_d_c_b_a(x3),
    ]
    .concat();
    out
}

#[cfg(any(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")), all(target_arch = "x86", not(target_feature = "sse2"))))]
pub unsafe fn _cubehash<R: Read>(input: &mut R, irounds: i32, frounds: i32, hashlen: i32) -> Vec<u8> {
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

pub fn cubehash<R: Read>(input: &mut R, revision: i32, hashlen: i32) -> Vec<u8> {
    unsafe {
        match if hashlen <= MAXHASHLEN && hashlen % 8 == 0 { revision }  else  { 0 } {
            3 => return _cubehash(input, 16, 32, hashlen),
            2 => return _cubehash(input, 160, 160, hashlen),
            _ => return Vec::new(),
        };
    }
}
