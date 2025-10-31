#![no_main]

extern crate libfuzzer_sys;

use arbitrary::Arbitrary;
use crate_batch_1::benchmark_vec_u8;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
pub struct Arguments {
    pub arg1: u8,
    pub arg2: u64,
    pub arg3: [u8; 64],
}

fuzz_target!(|arguments: Arguments| {
    // Convert arg1 to a Vec<u8>
    let arg1_vec = vec![arguments.arg1];
    benchmark_vec_u8(&arg1_vec, arguments.arg2, &arguments.arg3);
});
