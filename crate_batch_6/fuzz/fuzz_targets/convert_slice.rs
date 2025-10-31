#![no_main]

extern crate libfuzzer_sys;

use crate_batch_6::convert_slice;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    unsafe {
        convert_slice::<u8>(data);
    }
});
