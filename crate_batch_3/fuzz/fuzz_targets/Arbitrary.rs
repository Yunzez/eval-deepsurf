#![no_main]

extern crate libfuzzer_sys;

use arbitrary::Arbitrary;
use crate_batch_3::benchmark_string_operations;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
pub struct Arguments<'a> {
    pub arg1: &'a str,
    pub arg2: &'a [u8],
}

fuzz_target!(|arguments: Arguments| {
    // fuzzed code goes here
    benchmark_string_operations(arguments.arg1, arguments.arg2);
});
