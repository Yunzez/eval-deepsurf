# Disabled Dependencies

This file tracks dependencies that were commented out to keep the deepSURF
static analysis toolchain (based on `rustc_surf` / Rust 1.81.0-dev) working.

- `crate_batch_2` — disabled `kalker = "2.1.1"` because it depends on `rug`,
  which requires the unstable Cargo feature `edition2024`, incompatible with
  the `rustc_surf` toolchain.
- `crate_batch_2` — disabled `hyper`, `h2`, `jsonschema`, `juniper`, and `just`
  since their dependency stacks pull in `url`/`icu` releases that require
  Rust ≥ 1.82, beyond the `rustc_surf` toolchain target.
- `crate_batch_2` — disabled `geo = "0.29.3"` because it depends on
  `spade` ≥ 2.15, which uses the unstable `iter_repeat_n` API requiring a
  newer compiler than `rustc_surf` provides.
- `crate_batch_3` — disabled `naga` since it pulls in crates that require
  Rust ≥ 1.82.
- `crate_batch_3` — disabled the `rust-minidump`/`minidump-processor`
  dependencies because their stack (e.g., `backtrace`, ICU bindings) requires
  Rust ≥ 1.82.
- `crate_batch_4` — disabled `pgp = "0.15.0"` because it depends on
  `base64ct` with the unstable `edition2024` Cargo feature.
- `crate_batch_4` — disabled `pdf-112`/`pdf-115` (the `pdf` crate) because the
  dependency stack pulls in `backtrace` versions that require Rust ≥ 1.82.
- `crate_batch_4` — disabled the git dependency on `quick-xml` (v0.6) because
  it depends on `error-chain`/`backtrace` requiring Rust ≥ 1.82.
- `crate_batch_4` — disabled `phonenumber`, `prettytable`, and the git
  `pulldown-cmark` crates after stubbing out the dependent code paths that
  required Rust ≥ 1.82 (chrono/time-core).
- `crate_batch_4` — disabled the git `plist` dependency because it relies on
  `time-core` / `deranged` requiring unstable compiler features.
- `crate_batch_5` — disabled `sqlparser`, `sqlformat`, and `symbolic`, and
  replaced the associated benchmark code with stubs to avoid ICU/backtrace
  stacks that require Rust ≥ 1.82.
- `crate_batch_5` — pinned `backtrace` to v0.3.74 via git to keep AFL harness
  builds compatible with the rustc_surf toolchain.
- `crate_batch_6` — disabled `tokei` because it depends on `globset` 0.4.18,
  which requires the unstable Cargo feature `edition2024`.
