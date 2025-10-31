#![no_main]

extern crate libfuzzer_sys;

use arbitrary::Arbitrary;
use crate_batch_4::benchmark_vec_u8;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
pub struct Arguments<'a> {
    pub arg1: &'a [u8],
    pub arg2: &'a [u8],
}

fuzz_target!(|arguments: Arguments| {
    // fuzzed code goes here
    benchmark_vec_u8(arguments.arg1, arguments.arg2);
});
