use std::{f64, mem};

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;

fn main() {
    unsafe {
        let a = _mm256_set_pd(1.0, 2.0, 3.0, 4.0);
    }
    println!("Hello, world!");
}
