#![no_main]

extern crate libfuzzer_sys;

use crate_batch_6::get_arg_types;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    get_arg_types(data);
});
