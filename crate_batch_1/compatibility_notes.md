# crate_batch_1 compatibility notes (Rust 1.81.0-dev)

Date: 2025-11-01

We are running deepSURF with the custom `rustc_surf` toolchain (cargo 1.81.0-dev). Some popular crates recently raised their MSRV or adopted ICU/edition-2024 features that are not supported by this toolchain. To keep the batch building without invasive changes, we took the most conservative route: comment out only the problematic dependencies/usages.

Disabled dependencies (and why):
- alloy-json-abi (commented earlier)
  - Reason: pulls `ruint >= 1.17.0` which requires Cargo edition 2024.
- async-h1
  - Reason: via `http-types -> url 2.5.x -> idna_adapter/ICU`, needs Rust >= 1.82.
- flatgeobuf
  - Reason: via `reqwest -> url 2.5.x -> idna_adapter/ICU`, needs Rust >= 1.82.
- cursive
  - Reason: pulls `serde_json` which (current latest) pulls `indexmap 2.12` (MSRV >= 1.82).
- bson (three versions referenced: bson-10/11/12)
  - Reason: also pulls `serde_json -> indexmap 2.12` (MSRV >= 1.82).

Corresponding code disabled in `src/lib.rs`:
- Run 1 (already disabled): alloy-json-abi (edition-2024 path)
- Run 5: BSON decode examples (bson_10/11/12)
- Run 11: Cursive UI snippet
- Run 17: FlatGeobuf reader

Everything else remains intact (bcrypt, chrono, csscolorparser, cssparser, exmex, bincode variants, cookie, cranelift_reader, der/der_parser, fatfs, flac), unless they later surface incompatibilities.

Additionally added minimal safe wrappers to enable URAPI discovery:
- `pub fn entrypoint()` calls the existing `unsafe fn benchmark(..)`
- `pub fn entry_vec()` calls the existing `unsafe fn benchmark_vec_u8(..)`

These wrappers are public and safe, so deepSURF can register them as URAPIs (safe APIs that reach unsafe code). They don’t change crate behavior.

Reversibility:
- Each change is commented rather than removed. Re-enable by uncommenting the dependency lines in `Cargo.toml` and the corresponding code blocks in `src/lib.rs` when using a newer toolchain (Rust >= 1.83) or once deepSURF updates its `rustc_surf`.

Notes:
- We intentionally avoided version pinning/downgrades to keep the crate untouched beyond comments, per the minimal-change policy.
- If new MSRV errors appear, follow the same approach: identify the offending top-level dependency and temporarily comment it and its usage.
