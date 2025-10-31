# Running deepSURF on `crate_batch_{1-6}`

The steps below mirror the official deepSURF README and adapt them to this repository’s six crate batches. Follow the sequence end-to-end—skipping the static analysis phase or missing environment variables will leave the harness generator without the metadata it needs.

## 1. Prerequisites
- 64-bit Linux host (deepSURF is validated on Ubuntu 24.04 with AMD EPYC CPUs).
- Docker 24+ with at least 100 GB free disk, ≥64 GB RAM, and ≥10 CPU cores if you plan to fuzz at scale.
- An OpenRouter API key (LLM access). Export it before running any harness-generation command.
- `git`, `curl`, `rustup`, and `cargo` installed on the host.

> **Tip:** All example paths assume this repository lives at `/workspace/eval-deepsurf`. Adjust the paths if your checkout differs.

## 2. Build the deepSURF toolchain
1. Clone upstream deepSURF and enter the source tree:
   ```bash
   git clone https://github.com/purseclab/deepSURF.git ~/deepSURF
   cd ~/deepSURF/code
   ```
2. Let the helper script build the modified `rustc_surf` toolchain, harness generator, and afl.rs wrapper. This can take 30–60 minutes on a cold machine:
   ```bash
   ./install.sh
   ```
   - The script builds a Docker image (`deepsurf:latest`) and prepares the custom compiler under `~/.rustup/toolchains/rustc_surf`.
   - Alternatively, run `./build-docker.sh` followed by `./run-docker.sh` if you prefer to manage the container manually.
3. Whenever you start a new shell (inside or outside Docker), ensure the environment variables expected by the harness generator are exported:
   ```bash
   export SURF_HARNESS_GENERATOR_PATH=$HOME/deepSURF/code/harness_generator
   export GLOBAL_DATA_PATH=$HOME/deepSURF/code/global_data
   export SURF_ENABLE_VERBOSE=1
   export SURF_ENABLE_LLMS=1
   export SURF_DISABLE_REMOVE_LAST_TARGET=1
   export SURF_SKIP_OPTION="condskip"
   export SURF_ENABLE_OPTIMIZED_TREE_GEN=1
   export OPENROUTER_API_KEY="sk-or-..."
   # Optional overrides:
   # export LLM_BACKEND="deepseek/deepseek-r1"
   # export SURF_DISABLE_LLM_DOCUMENTATION=1  # When prompts exceed context limits
   ```

## 3. Prepare the crate batches
1. Make the current repository visible inside the container (or host shell) that runs deepSURF. Using Docker:
   ```bash
   ./run-docker.sh \
     --workspace /workspace/eval-deepsurf \
     --mount /workspace/eval-deepsurf:/workspace/eval-deepsurf
   ```
   Inside the container, the crate directories are available at `/workspace/eval-deepsurf/crate_batch_*`.
2. Each `crate_batch_n` already exposes only `unsafe fn` entry points (per the modifications in `src/lib.rs`). No additional code changes are required before analysis.
3. Optional: `cargo fetch` each crate once to prime the dependency cache and shorten later builds.

## 4. Static analysis & harness generation
Run the two-phase process (analysis + harness generation) for each crate. Replace `N` with `1` through `6` in the snippets below.

```bash
# Phase A: Static analysis with rustc_surf
export SURF_ANALYZE_LIB=1
export SURF_WORKING_PATH=/workspace/eval-deepsurf/crate_batch_N
cd $SURF_WORKING_PATH
cargo clean
RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
unset SURF_ANALYZE_LIB

# Phase B: Harness generation (LLM-augmented)
cd $SURF_HARNESS_GENERATOR_PATH
URAPI_JSON=$(ls "$SURF_WORKING_PATH/deepSURF/report/"*.urapi.json | head -n1)
TARGET_ID=${URAPI_JSON%.urapi.json}
RUSTFLAGS="-Awarnings" cargo run --release "$TARGET_ID"
```

### Notes
- The static build step emits deepSURF metadata beneath `<crate>/deepSURF/report/`. Do not delete this directory; the harness generator ingests it on the next step.
- If a crate requires Cargo features or extra dependencies, export `SURF_ENABLE_FEATURES="..."` or `SURF_EXTRA_DEPS="crate=version"` before running the harness generator (mirroring the patterns in `~/deepSURF/generate_harnesses.sh`).
- When an LLM augmentation attempt fails repeatedly, the generator falls back to the statically produced harness. Re-run the `cargo run` command to retry augmentation after refining prompts or increasing `SURF_MAX_RETRIES`.

## 5. Inspect generated harnesses
- Output lives under `<crate>/deepSURF/fuzz/`. With `SURF_SKIP_OPTION=condskip`, expect a layout similar to:
  ```
  crate_batch_N/deepSURF/
    ├── report/                 # Static analysis artifacts (.urapi.json, stats)
    ├── fuzz/
    │   ├── condskip/
    │   │   ├── targets/<target-name>/Cargo.toml
    │   │   └── targets/<target-name>/src/main.rs   # AFL-compatible harness
    │   └── llm-only/templates/…                    # Prompt templates & retries
    └── logs/                                       # Generator logs
  ```
- The harness `Cargo.toml` already depends on `afl`/`afl-instrumentation`. No manual edits should be necessary unless you want to pin features.

## 6. Fuzz the harnesses
Run AFL++ (via `cargo afl`) from each harness directory. The example below mirrors the paper’s dual-run configuration (ASan + CmpLog workers):

```bash
# Inside one harness directory, e.g.
cd /workspace/eval-deepsurf/crate_batch_N/deepSURF/fuzz/condskip/targets/<target>

# Build once with AddressSanitizer
RUSTFLAGS="-Zsanitizer=address" \
  cargo afl build --release --target x86_64-unknown-linux-gnu

# Launch two fuzzing jobs sharing the same corpus
mkdir -p findings findings-asan findings-cmplog inputs
echo "seed" > inputs/seed

# Worker 1: ASan
cargo afl fuzz -i inputs -o findings-asan target/release/<target> -- @@

# Worker 2: CmpLog (new terminal)
AFLOPTS="-c 0" cargo afl fuzz \
  -i inputs -o findings-cmplog \
  -S cmplog --target=cmp \
  target/release/<target> -- @@
```

- Merge findings (crashes, hangs, queue) into a shared `findings` directory as needed.
- For long campaigns, pin each worker to a dedicated CPU core and monitor with `cargo afl tmin/showmap` for coverage growth.

## 7. Post-processing
- Crashes are written under each harness’s `findings-*` folder. Use `cargo afl cmin`, `cargo afl tmin`, or `asan_symbolize.py` to triage.
- The harness generator also produces per-URAPI statistics in `<crate>/deepSURF/report/*.stats.txt`, including URAPI coverage and augmentation status.
- When you find a reproducer, reduce it to a minimal input and report per RustSec guidelines.

## 8. Batch automation (optional)
A simple loop will process all six crates in one go:
```bash
for N in 1 2 3 4 5 6; do
  export SURF_WORKING_PATH=/workspace/eval-deepsurf/crate_batch_${N}
  export SURF_ANALYZE_LIB=1
  (cd "$SURF_WORKING_PATH" && cargo clean && \
   RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build)
  unset SURF_ANALYZE_LIB
  URAPI_JSON=$(ls "$SURF_WORKING_PATH/deepSURF/report/"*.urapi.json | head -n1)
  TARGET_ID=${URAPI_JSON%.urapi.json}
  (cd "$SURF_HARNESS_GENERATOR_PATH" && \
   RUSTFLAGS="-Awarnings" cargo run --release "$TARGET_ID")
done
```

Keep an eye on API usage: each crate may trigger dozens of augmentations, so budget your OpenRouter quota accordingly.

---
With these steps you can reproduce the deepSURF pipeline for `crate_batch_1` through `crate_batch_6`, generate AFL-compatible harnesses, and begin fuzzing unsafe Rust code that now exposes explicit `unsafe fn` entry points.
