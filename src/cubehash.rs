use std::io::Read;
use std::io::Stdin;
use std::ptr;
use core::arch::x86_64::__m128i;
use core::arch::x86_64::_mm_set_epi32;
use core::arch::x86_64::_mm_xor_si128;
use core::arch::x86_64::_mm_loadu_si128;
use core::arch::x86_64::_mm_add_epi32;
use core::arch::x86_64::_mm_shuffle_epi32;
use core::arch::x86_64::_mm_slli_epi32;
use core::arch::x86_64::_mm_srli_epi32;

const BUFSIZE: i32 = 65536;
const ROUNDS: i32 = 16;
const BLOCKSIZE: i32 = 32;
const MAXHASHLEN: i32 = 512;

unsafe fn print_value(value: __m128i, tag: &str, index: i32, limit: i32) {
    if index < limit {
        let buffer = std::mem::transmute::<__m128i, [u64; 2]>(value);
        print!("\n{} {:016x} {:016x}", tag, buffer[1], buffer[0]);
    }
}

pub unsafe fn _cubehash(mut input: &mut Stdin, irounds: i32, frounds: i32, hashlen: i32) -> Vec<u8> {
    //eprintln!("Hashing using CubeHash{}+16/32+{}-{}...", irounds, frounds, hashlen);
    
    let mut done = false;
    let mut eof = false;
    let mut more = true;
    //let mut data: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize]; // What's the best way to initialize this??

    let mut data = {
        let mut data: [std::mem::MaybeUninit<u8>; BUFSIZE as usize] = std::mem::MaybeUninit::uninit().assume_init();

        //for elem in &mut data[..] {
        //    std::ptr::write(elem.as_mut_ptr(), 0);
        //}

        for i in 0..BUFSIZE {
            data[i as usize] = std::mem::MaybeUninit::new(0);
        }

        std::mem::transmute::<_, [u8; BUFSIZE as usize]>(data)
    };

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
    
    // equivalent to a memset, not sure what's best here
    //ptr::write_bytes(&mut data, 0, datasize as usize); // segmentation fault here!
    //for i in &mut data[0..datasize as usize] { *i = 0 }

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

                //print_value(x0, "x0: ", 0, 1);
                //print_value(x1, "x1: ", 0, 1);
                //print_value(x2, "x2: ", 0, 1);
                //print_value(x3, "x3: ", 0, 1);
            }
        }

        done = !more;

        if more {
            if eof {
                datasize = frounds / ROUNDS * BLOCKSIZE;
                ptr::write_bytes(&mut input, 0, datasize as usize);
                x7 = _mm_xor_si128(x7, _mm_set_epi32(0, 1, 0, 0));
                more = false;
            } else {
                datasize = input.read(&mut data).unwrap() as i32;

                if datasize < BUFSIZE {
                    let padsize = BLOCKSIZE - datasize % BLOCKSIZE;
                    ptr::write_bytes(&mut data[datasize as usize], 0, padsize as usize);
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

pub fn cubehash(input: &mut Stdin, revision: i32, hashlen: i32) -> Vec<u8> {
    unsafe {
        match if hashlen <= MAXHASHLEN && hashlen % 8 == 0 { revision }  else  { 0 } {
            3 => return _cubehash(input, 16, 32, hashlen),
            2 => return _cubehash(input, 160, 160, hashlen),
            _ => return Vec::new(),
        };
    }
}