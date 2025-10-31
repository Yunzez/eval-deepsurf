#![no_main]

extern crate libfuzzer_sys;

use crate_batch_5::test_decode;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    test_decode(data);
});
