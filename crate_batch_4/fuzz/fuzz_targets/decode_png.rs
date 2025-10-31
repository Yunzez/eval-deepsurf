#![no_main]

extern crate libfuzzer_sys;

use crate_batch_4::decode_png;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    decode_png(data);
});
