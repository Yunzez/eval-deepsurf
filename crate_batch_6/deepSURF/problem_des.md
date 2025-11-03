## Harness issues observed in crate_batch_6

Only crate_batch_5 surfaced crashes; digging into the crate_batch_6 fuzzers explains why. The generated harnesses there keep AFL from ever exercising the buggy code paths:

- `crate_batch_6_fuzz_17997780617928745793`: ignores the fuzz input entirely and always calls `benchmark_misc()`, which operates on constant data. Coverage never changes, so the fuzzer cannot make progress.
- `crate_batch_6_fuzz_17997780617928745793_gpt-4o_turn1`: spins in an infinite `loop` and usually hits branch 0, where `get_arg_types(first_half)` almost always returns `None`. `_unwrap_option` then calls `process::exit(0)`, so AFL treats the run as a clean exit and stops mutating that seed.
- `crate_batch_6_fuzz_9306830355928128974_gpt-4o_turn4`: repeats the same `_unwrap_option` pattern on unvalidated slices and aborts whenever UTF-8 decoding fails. The interesting bytes are discarded before they reach the targets.
- `crate_batch_6_fuzz_5130697570958458892_gpt-4o_turn2`: does feed data in, but it truncates vectors to near-zero length, prints them, and skips most work when helpers return `None`. It rarely constructs the structured inputs that would drive `BenchmarkData`, `convert_slice`, or the font/zip/parsing code.

`crate_batch_6/src/lib.rs` still contains plenty of hazardous operations (unsafe slice casts, font parsing with a panicking outline builder, `Message::from_raw`, etc.), so better harnesses would likely recover crashes. As written, these fuzzers make a good case study of how small harness flaws can completely neuter a fuzz campaign.
