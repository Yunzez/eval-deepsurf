#![no_main]

extern crate libfuzzer_sys;

use arbitrary::Arbitrary;
use crate_batch_6::benchmark_template_and_strings;
use libfuzzer_sys::fuzz_target;

#[derive(Arbitrary, Debug)]
pub struct Arguments<'a> {
    pub arg1: &'a str,
    pub arg2: &'a str,
    pub arg3: u64,
}

fuzz_target!(|arguments: Arguments| {
    // fuzzed code goes here
    benchmark_template_and_strings(arguments.arg1, arguments.arg2, arguments.arg3);
});
