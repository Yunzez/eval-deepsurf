# Progress Log

## 2025-11-01
- Reviewed `running.md` and validated env requirements.
- Loaded OpenRouter API key from `deepSURF/generate_harnesses.sh`; will use `LLM_BACKEND=openai/gpt-4o`.
- Started persistent container `deepsurf-work`; waited for entry build to finish.
- First analysis attempts failed due to Cargo edition 2024 via `ruint` (transitive from `alloy-json-abi`). Per minimal-change policy, commented out `alloy-json-abi` and its usage.
- Hit MSRV bumps via ICU/idna_adapter and indexmap 2.12. Commented out only the offending deps and their usages:
	- Disabled in `crate_batch_1/Cargo.toml`: `async-h1`, `flatgeobuf`, `cursive`, `bson` (10/11/12).
	- Disabled in `src/lib.rs`: runs 5 (BSON), 11 (cursive), 17 (flatgeobuf). Kept all other runs.
- Added `crate_batch_1/compatibility_notes.md` documenting the temporary disables and rationale.
- Ran static analysis entirely inside the container:
	- `export SURF_ANALYZE_LIB=1 SURF_WORKING_PATH=/workspace/crate_batch_1`
	- `cd /workspace/crate_batch_1 && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build`
	- Result: Success; deepSURF artifacts emitted at `/workspace/crate_batch_1/deepSURF/report/`.
- Note: Avoid running `cargo clean/build` on host on this crate: `target/` is root-owned from prior container builds causing permission denied. Use container-only flow or remove `target/` from inside the container.

Next steps:
- Run harness generator with gpt-4o: export `OPENROUTER_API_KEY` and `LLM_BACKEND=openai/gpt-4o`, then from `/workspace/deepSURF/code/harness_generator` run `cargo run --release <TARGET_ID>` where `<TARGET_ID>` is the basename of the URAPI JSON in `/workspace/crate_batch_1/deepSURF/report/` without the `.urapi.json` suffix.

## 2025-11-02
- Resumed with crate batches 5 & 6; reran static analysis inside `deepsurf-work` to confirm `SURF_ANALYZE_LIB` outputs after our new unsafe blocks.
- Commented out high-MSRV dependencies in `crate_batch_5` (`sqlparser`, `sqlformat`, `symbolic`) and stubbed out the associated runs; added a workspace-level patch and `.cargo/config.toml` to pin `backtrace` to 0.3.74 so AFL harnesses compile under rustc_surf.
- Adjusted `crate_batch_5` harness generation flow: cleaned stale `fuzz/llm` state, regenerated no-LLM targets, and ran the LLM pass with `openai/gpt-4o`. Final stats show 7/7 URAPIs covered (5 improved via LLM); fuzzing targets landed in `deepSURF/fuzz/fuzzing_corpus/`.
- For `crate_batch_6`, disabled the `tokei` run (globset edition2024 issue) and dependency, then reran static analysis and full harness generation. LLM stage now covers 7/7 URAPIs with two enhanced targets promoted to the fuzz corpus.
- Updated `disabled_dependencies.md` with the new exclusions and backtrace pin; recorded global patch in `.cargo/config.toml`.
