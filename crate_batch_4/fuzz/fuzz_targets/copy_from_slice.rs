#![no_main]

extern crate libfuzzer_sys;

use crate_batch_4::align_from_u8;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if data.len() > 8 {
        let mut input = [0u8; 8];
        input.copy_from_slice(&data[..8]);
        let val = input[0]; // Extract the first byte
        align_from_u8(val);
    }
});
