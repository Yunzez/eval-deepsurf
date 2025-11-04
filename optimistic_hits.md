# Optimistic Run Coverage per Crate

## crate_batch_1
- Harness `crate_batch_1/deepSURF/fuzz/fuzzing_corpus/crate_batch_1_fuzz_16026286153529657936_gpt-4o_turn4/src/crate_batch_1_fuzz_16026286153529657936.rs`
  - Drives `benchmark_string` (`crate_batch_1/src/lib.rs:105-158`), hitting runs 3 (bcrypt::verify), 6_part1 (chrono_16 RFC‑2822 parser), 9 (csscolorparser::parse), 10 (cssparser token stream), and 14 where `exmex::parse_with_default_ops::<f64>(str).unwrap()` yields the deterministic panic.
  - Also reaches `benchmark_vec_u8` (`crate_batch_1/src/lib.rs:165-212`), exercising runs 4 (dual bincode decoders), 6_part2 (chrono checked_add_days), 7 (cookie::CookieJar signing), 8 (cranelift_reader::parse_test on fuzzed UTF‑8), and 12 (DER decoder). Any of these runs can reproduce their historical trophies once AFL++ mutates into the right formats.

## crate_batch_2
- Harness `crate_batch_2/deepSURF/fuzz/fuzzing_corpus/crate_batch_2_fuzz_5013163950312134626_gpt-4o_turn1/src/crate_batch_2_fuzz_5013163950312134626.rs`
  - Calls `benchmark_vec_u8` (`crate_batch_2/src/lib.rs:107-174`), covering runs 2 (fontdue font loader), 4 (gimli::Index::parse_sysv_index), 6 (serde_hjson::from_slice), 9 (image::webp decoder), and 10 (gif::DecodeOptions). These are exactly the trophy cases curated for this crate.
  - Calls `benchmark_strings` (`crate_batch_2/src/lib.rs:178-227`), hitting runs 5 (handlebars templating stress), 7 (addr::parser::Name parsing), and 11 (jpeg decoder plus fancy_regex creation).
  - Calls `benchmark_artifacts` (`crate_batch_2/src/lib.rs:90-105`), i.e., run 1, whose `read_dir` / `File::open` unwrap chain still panics immediately on directory entries.
- Harness `crate_batch_2/deepSURF/fuzz/fuzzing_corpus/crate_batch_2_fuzz_4675919745659301435_gpt-4o_turn1/src/crate_batch_2_fuzz_4675919745659301435.rs` loops across the same three functions, so it exercises the identical trophy run set {1,2,4,5,6,7,9,10,11}.

## crate_batch_3
- Harness `crate_batch_3/deepSURF/fuzz/fuzzing_corpus/crate_batch_3_fuzz_12228132628757202096_gpt-4o_turn1/src/crate_batch_3_fuzz_12228132628757202096.rs`
  - Dispatches into `main` → `benchmark` (`crate_batch_3/src/lib.rs:101-137`), thus reaching `benchmark_vec_u8` runs 1 (deflate decoder), 2 (lopdf::Document::load_mem), 3 (lz4_fear frame reader), 4 (lz4_flex round‑trip), 9 (mp4ameta::Tag), 12 (nom parser01 on 1024‑byte buffers), 13 (npy::from_bytes), and 14 (ntfs::Ntfs::read_upcase_table).
  - The harness’ op=1 path re-enters `benchmark_vec_u8`, reinforcing the same trophy coverage, while op=2 triggers `benchmark_string_operations` (run 5 scaffolding) whenever those blocks are re-enabled.

## crate_batch_4
- Harness `crate_batch_4/deepSURF/fuzz/fuzzing_corpus/crate_batch_4_fuzz_16422819160411493718/src/crate_batch_4_fuzz_16422819160411493718.rs` (`DefId(0:48)`) targets `decode_png` via `benchmark_vec_u8`, reaching runs 2 and 8 (`crate_batch_4/src/lib.rs:139-210`)—pcapng parsing plus `decode_png`’s guarded decoder path that has historically triggered crashes with malformed PNG chunks.
- Harness `crate_batch_4/deepSURF/fuzz/fuzzing_corpus/crate_batch_4_fuzz_3849529532662044554/src/crate_batch_4_fuzz_3849529532662044554.rs` (`DefId(0:46)`) drives `benchmark_strings` (currently logs “disabled”, but any re-enabled block would be exercised).
- Harness `crate_batch_4/deepSURF/fuzz/fuzzing_corpus/crate_batch_4_fuzz_8746160299732535655/src/crate_batch_4_fuzz_8746160299732535655.rs` (`DefId(0:44)`) runs `benchmark_vec_u8` with the same run coverage (2 and 8), ensuring both pcapng and PNG trophies stay reachable.
- Harness `crate_batch_4/deepSURF/fuzz/fuzzing_corpus/crate_batch_4_fuzz_5927952589163955769/src/crate_batch_4_fuzz_5927952589163955769.rs` maps to the same entry points, so the combined set of runs reachable from the corpus is {run 1 via `benchmark_misc` when invoked, run 2 via `parse_block`, and run 8 via `decode_png`}.

## crate_batch_5
- Harness `crate_batch_5/deepSURF/fuzz/fuzzing_corpus/crate_batch_5_fuzz_1006074549953565885_gpt-4o_turn2/src/crate_batch_5_fuzz_1006074549953565885.rs`
  - Reaches `benchmark_numeric` run 3 (`crate_batch_5/src/lib.rs:245-266`) for the RON parser trophies.
  - Drives `benchmark_string_ops` runs 4, 5, 6, 13, and 18 (`crate_batch_5/src/lib.rs:275-327`), with run 4’s `Ini::read_from(&mut cursor).unwrap()` providing the fastest crash.
  - Reuses `benchmark_vec_u8` runs 1, 7, 8, 9, 14, 15, and 17 (`crate_batch_5/src/lib.rs:157-241`), covering CBOR, YAML, ASN.1, brotli, ssh, and torrent trophies.
  - Touches `benchmark_misc` run 11 (`crate_batch_5/src/lib.rs:331-349`), recreating the Soroban budget comparison trophy.
- Harness `crate_batch_5/deepSURF/fuzz/fuzzing_corpus/crate_batch_5_fuzz_18413384834626210455_gpt-4o_turn1/src/crate_batch_5_fuzz_18413384834626210455.rs` walks the same functions and additionally invokes `test_decode` before the benchmark, so it stresses run 17’s bittorrent parser both pre- and post-benchmark.

## crate_batch_6
- Harness `crate_batch_6/deepSURF/fuzz/fuzzing_corpus/crate_batch_6_fuzz_9306830355928128974/src/crate_batch_6_fuzz_9306830355928128974.rs` calls `crate_batch_6::main`, so it covers every run reachable via `benchmark_template_and_strings` (1, 4, 6, 7, 9, 10), `benchmark_vec_u8` (2, 5, 12, 13, 14, 15, 16), and `benchmark_misc` (11) in `crate_batch_6/src/lib.rs`.
- Harness `crate_batch_6/deepSURF/fuzz/fuzzing_corpus/crate_batch_6_fuzz_3751115272684356604/src/crate_batch_6_fuzz_3751115272684356604.rs` concentrates on `benchmark_template_and_strings` (`crate_batch_6/src/lib.rs:180-279`), with run 4’s `toml::from_str(&first_input).unwrap()` being the easiest panic and runs 6/7/9/10 covering ByteUnit, UnicodeSegmentation, Url, and Uuid trophies.
- Harness `crate_batch_6/deepSURF/fuzz/fuzzing_corpus/crate_batch_6_fuzz_5130697570958458892/src/crate_batch_6_fuzz_5130697570958458892.rs` sends fuzz data to `benchmark_vec_u8` (`crate_batch_6/src/lib.rs:281-372`), reaching runs 2 (todo.txt parser), 5 (ttf_parser outline), 12 (vobsub subtitles), 13 (wayland Message::from_raw with FD conversion), 14 (ws::Frame::parse), 15 (yaxpeax decoder), and 16 (zip::ZipArchive with `by_index(i).unwrap()`).
- Harness `crate_batch_6/deepSURF/fuzz/fuzzing_corpus/crate_batch_6_fuzz_17997780617928745793/src/crate_batch_6_fuzz_17997780617928745793.rs` isolates `benchmark_misc` run 11 (`crate_batch_6/src/lib.rs:376-389`), repeatedly exercising `vial::Request::from_reader`.
- Harness `crate_batch_6/deepSURF/fuzz/fuzzing_corpus/crate_batch_6_fuzz_18387544885740318548/src/crate_batch_6_fuzz_18387544885740318548.rs` targets `get_arg_types` (`crate_batch_6/src/lib.rs:148-171`); once the 16-byte selector is valid it dovetails with the `benchmark_vec_u8` harness to reach run 13’s wayland message handling.
