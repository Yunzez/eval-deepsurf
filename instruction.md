# How to run deepSURF on crate_batch_N

This repo includes six batches: `crate_batch_1` … `crate_batch_6`. Follow these steps to analyze any `crate_batch_N` and generate fuzz harnesses, using OpenRouter with the OpenAI gpt-4o model.

## One-time
- Ensure Docker image `deepsurf:latest` is built (you said it is).
- The OpenRouter key is stored in `deepSURF/generate_harnesses.sh` as an `export OPENROUTER_API_KEY=...` line. We will source just this line.

## Start a persistent container (recommended)
Keep a long-running container so the toolchain isn’t rebuilt each run.

```bash
# Start once; reuse across sessions
docker run -d --name deepsurf-work \
  -v "$PWD":/workspace -w /workspace \
  deepsurf:latest tail -f /dev/null

# Watch until you see: "[entry] deepSURF is ready!"
docker logs -f deepsurf-work
```

Note: The image’s entrypoint builds and wires custom toolchains on first start (can take ~1h). It also `cd`s to `/root` and does not cd back; that’s why commands below always change to the target directory explicitly.

## Environment for this session
Use the same LLM/model for all runs: OpenAI gpt-4o via OpenRouter.

```bash
# In your host shell
source <(grep -m1 '^export OPENROUTER_API_KEY' deepSURF/generate_harnesses.sh)
export LLM_BACKEND="openai/gpt-4o"
export SURF_HARNESS_GENERATOR_PATH=$HOME/deepSURF/code/harness_generator
export GLOBAL_DATA_PATH=$HOME/deepSURF/code/global_data
export SURF_ENABLE_VERBOSE=1
export SURF_ENABLE_LLMS=1
export SURF_DISABLE_REMOVE_LAST_TARGET=1
export SURF_SKIP_OPTION="condskip"
export SURF_ENABLE_OPTIMIZED_TREE_GEN=1
```

You can pass these as `-e` flags to `docker exec` or run a shell with them inlined.

## Run static analysis for crate_batch_N
```bash
N=1   # change to 2..6 for other batches

# Inside the container (recommended):
docker exec -e SURF_ANALYZE_LIB=1 -it deepsurf-work bash -lc "set -euo pipefail; cd /workspace/crate_batch_${N}; pwd; ls -1 Cargo.toml; cargo clean; RUSTFLAGS='-Zub-checks=no -Awarnings' cargo +rustc_surf build"
```

This step emits deepSURF metadata into `/workspace/crate_batch_${N}/deepSURF/report/*.urapi.json`.

## Generate harnesses (LLM-augmented)
```bash
N=1   # same N as above

docker exec \
  -e OPENROUTER_API_KEY="$OPENROUTER_API_KEY" \
  -e LLM_BACKEND="$LLM_BACKEND" \
  -e SURF_HARNESS_GENERATOR_PATH="$SURF_HARNESS_GENERATOR_PATH" \
  -e GLOBAL_DATA_PATH="$GLOBAL_DATA_PATH" \
  -it deepsurf-work bash -lc "\
    set -euo pipefail; \
    export SURF_WORKING_PATH=/workspace/crate_batch_${N}; \
    URAPI_JSON=\$(ls \"$SURF_WORKING_PATH/deepSURF/report\"/*.urapi.json | head -n1); \
    TARGET_ID=\${URAPI_JSON%.urapi.json}; \
    cd \"$SURF_HARNESS_GENERATOR_PATH\"; \
    RUSTFLAGS='-Awarnings' cargo run --release \"$TARGET_ID\" \
  "
```

Outputs go under `/workspace/crate_batch_${N}/deepSURF/fuzz/`.

## Optional: Fuzz a harness
```bash
N=1
TARGET_DIR=$(find \
  "/home/yzzhao3/eval-deepsurf/crate_batch_${N}/deepSURF/fuzz/condskip/targets" \
  -maxdepth 1 -mindepth 1 -type d | head -n1)

docker exec -it deepsurf-work bash -lc "\
  set -euo pipefail; \
  cd \"${TARGET_DIR}\"; \
  RUSTFLAGS='-Zsanitizer=address' cargo afl build --release --target x86_64-unknown-linux-gnu; \
  mkdir -p inputs findings-asan; echo seed > inputs/seed; \
  cargo afl fuzz -i inputs -o findings-asan target/release/$(basename \"${TARGET_DIR}\") -- @@ \
"
```

## Troubleshooting
- error: "could not find Cargo.toml in /root": the container entrypoint `cd`’s to `/root`. Always `cd /workspace/crate_batch_N` inside the exec’d shell as shown above.
- If no `*.urapi.json` appears after the build, ensure `SURF_ANALYZE_LIB=1` was set for the static analysis step and that you used `cargo +rustc_surf build`.
- If LLM calls fail: verify `OPENROUTER_API_KEY` is exported and `LLM_BACKEND` is set to `openai/gpt-4o`.

With this, you can run any `crate_batch_N` end-to-end without editing the crate sources.