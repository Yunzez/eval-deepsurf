#![no_main]

extern crate libfuzzer_sys;

use crate_batch_5::benchmark_numeric;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if data.len() > 8 {
        let mut input = [0u8; 8];
        input.copy_from_slice(&data[..8]);
        let val = u64::from_le_bytes(input);
        benchmark_numeric(val);
    }
});
