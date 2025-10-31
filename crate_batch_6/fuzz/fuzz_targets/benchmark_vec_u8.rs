#![no_main]

extern crate libfuzzer_sys;

use crate_batch_6::benchmark_vec_u8;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    benchmark_vec_u8(data);
});
