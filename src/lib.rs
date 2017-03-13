#![feature(collections)]
#![feature(core_intrinsics)]
#![no_std]


extern crate libc;
extern crate collections;

extern crate approx_eq;
extern crate signed;


mod float;


pub use float::Float;
