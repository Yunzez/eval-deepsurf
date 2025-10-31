#![no_main]

extern crate libfuzzer_sys;

use crate_batch_2::benchmark;
use crate_batch_2::BenchmarkData; // Assuming BenchmarkData is defined in crate_batch_2
use libfuzzer_sys::fuzz_target;

trait FromStr {
    fn from_str(input: &str) -> Option<Self>
    where
        Self: Sized;
}

impl FromStr for BenchmarkData {
    fn from_str(input: &str) -> Option<BenchmarkData> {
        // Conversion logic from &str to BenchmarkData
        // This is a placeholder and should be replaced with actual logic
        Some(BenchmarkData {
            testKey: [0; 64],           // Default value for [u8; 64]
            testString: String::new(),  // Default value
            testString2: String::new(), // Default value
            testU64: 0,                 // Default value for u64
            testVecU8: Vec::new(),      // Default value for Vec<u8>
                                        // Add other fields with default values
        })
    }
}

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if let Ok(input) = std::str::from_utf8(data) {
        if let Some(benchmark_data) = BenchmarkData::from_str(input) {
            benchmark(&benchmark_data);
        }
    }
});
