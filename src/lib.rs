// Any copyright is dedicated to the Public Domain.
// http://creativecommons.org/publicdomain/zero/1.0/

#![feature(stdsimd)]

extern crate stdsimd;
extern crate simd;

use stdsimd::simd::b8x16;
use simd::bool8ix16;

#[no_mangle]
pub fn stdsimd_any_8x16(b: b8x16) -> bool {
    b.any()
}

#[no_mangle]
pub fn stdsimd_all_8x16(b: b8x16) -> bool {
    b.all()
}

#[no_mangle]
pub fn simd_any_8x16(b: bool8ix16) -> bool {
    b.any()
}

#[no_mangle]
pub fn simd_all_8x16(b: bool8ix16) -> bool {
    b.all()
}
