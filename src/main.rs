use std::{f64, mem};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn main() {
    unsafe {
        println!("-------------------------------------------");
        let four_zeros = _mm_setzero_ps();
        let four_ones = _mm_set1_ps(1.0);
        let four_floats = _mm_set_ps(1.0, 2.0, 3.0, 4.0);
        println!("{:?}",four_zeros);
        println!("{:?}",four_ones);
        println!("{:?}",four_floats);
    }
    unsafe {
        println!("-------------------------------------------");
        let all_bytes_zero = _mm_setzero_si128();
        let all_bytes_one = _mm_set1_epi8(1);
        let four_i32 = _mm_set_epi32(1, 2, 3, 4);
        println!("{:?}",all_bytes_zero);
        println!("{:?}",all_bytes_one);
        println!("{:?}",four_i32);
    }
    unsafe {
        println!("-------------------------------------------");
        let a = _mm256_set_pd(1.0, 2.0, 3.0, 4.0);
        let b = _mm256_set_pd(1.0, 2.0, 3.0, 4.0);
        let term = _mm256_mul_pd(a, b);
        println!("{:?}",term);
        let term_unpacked: (f64, f64, f64, f64) = mem::transmute(term);
        println!("{:?}",term_unpacked);
        println!("{:?}",term_unpacked.0);
    }

    unsafe {
        println!("-------------------------------------------");
        let a = _mm_set_epi32(1, 2, 3, 4);
        let b = _mm_set_epi32(1, 2, 3, 4);
        let c = _mm_mul_epu32(a, b);
        let d = _mm_add_epi32(a, b);
        println!("{:?}", a);
        println!("{:?}", b);
        println!("{:?}", c);
        println!("{:?}", d);
    }

    unsafe {
        println!("-------------------------------------------");
        let a = _mm256_set_epi32(1 as i32, 2 as i32, 3 as i32, 4 as i32, 5 as i32, 6 as i32, 7 as i32, 8 as i32);
        let b = _mm256_set_epi32(1 as i32, 2 as i32, 3 as i32, 4 as i32, 5 as i32, 6 as i32, 7 as i32, 8 as i32);
        let c = _mm256_add_epi32(a,b);
        println!("{:?}", c);
        //let c2: (i32, i32, i32, i32) = mem::transmute(c);
        //println!("{:?}", c2);
        //let d = _mm256_blend_epi32(a,b,0);
        //let d2: (i32, i32, i32, i32) = mem::transmute(d);
        //println!("{:?}", d2);
    }
    println!("Hello, world!");
}
