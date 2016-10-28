#![feature(asm)]
#![allow(non_snake_case)]
use std::sync::*;
use std::thread;
use std::cell::*;
use std::marker;
extern crate rand;

struct U8(UnsafeCell<u8>);
impl U8 {
    fn new(i: u8) -> Self {
        U8(UnsafeCell::new(i))
    }
}
unsafe impl marker::Sync for U8 {}
unsafe impl marker::Send for U8 {}

fn main() {
    let start = Arc::new(Barrier::new(3));
    let X = Arc::new(U8::new(0));
    let Y = Arc::new(U8::new(0));
    let r1 = Arc::new(U8::new(0));
    let r2 = Arc::new(U8::new(0));
    let end = Arc::new(Barrier::new(3));

    let start1 = start.clone();
    let X1 = X.clone();
    let Y1 = Y.clone();
    let r11 = r1.clone();
    let end1 = end.clone();
    thread::spawn(move|| {
        let X = X1.0.get();
        let Y = Y1.0.get();
        let r1 = r11.0.get();
        loop {
            start1.wait();
            while rand::random::<u32>() % 8 != 0 {}
            unsafe {
                *X = 1;
                // swap these two lines for a `mfence` and not just compiler reordering
                // asm!("mfence" ::: "memory" : "volatile");
                asm!("" ::: "memory" : "volatile");
                *r1 = *Y;
            }
            end1.wait();
        }
    });

    let start2 = start.clone();
    let X2 = X.clone();
    let Y2 = Y.clone();
    let r22 = r2.clone();
    let end2 = end.clone();
    thread::spawn(move|| {
        let X = X2.0.get();
        let Y = Y2.0.get();
        let r2 = r22.0.get();
        loop {
            start2.wait();
            while rand::random::<u32>() % 8 != 0 {}
            unsafe {
                *Y = 1;
                // swap these two lines for a `mfence` and not just compiler reordering
                // asm!("mfence" ::: "memory" : "volatile");
                asm!("" ::: "memory" : "volatile");
                *r2 = *X;
            }
            end2.wait();
        }
    });

    let mut detected = 0;
    let mut iterations = 1;
    let X = X.0.get();
    let Y = Y.0.get();
    let r1 = r1.0.get();
    let r2 = r2.0.get();
    loop {
        unsafe {
            *X = 0;
            *Y = 0;
        }
        start.wait();
        end.wait();
        if unsafe {*r1 == 0 && *r2 == 0} {
            detected += 1;
            println!("{} reorders detected after {} iterations", detected, iterations);
        }
        iterations += 1;
    }
}
