# deepSURF quickstart for crate_batch_{1..6}

This condenses the upstream docs and uses commands we verified locally. The key is: enter the Docker shell first, set SURF_WORKING_PATH to the crate (e.g., /workspace/crate_batch_1), run static analysis, then run the harness generator.

## 0) One-time setup (outside Docker)
- Build/run the official image and container from the upstream repo (paths may vary):
```bash
cd deepSURF/code
./build-docker.sh            # builds deepsurf:latest
./run-docker.sh              # starts container (e.g., deepsurf-work) and mounts your repo at /workspace
```

## 1) Enter the container shell
```bash
docker exec -it deepsurf-work bash
```

All commands below run inside that shell.

## 2) Static analysis (per crate)
Replace N with 1..6. For example, crate_batch_1:
```bash
export SURF_ANALYZE_LIB=1
export SURF_WORKING_PATH=/workspace/crate_batch_1
cd "$SURF_WORKING_PATH"
cargo clean
RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
unset SURF_ANALYZE_LIB
```
This produces report files under /workspace/crate_batch_1/deepSURF/report/*.urapi.json

## 3) Harness generation
Minimal static-only (no LLM needed):
```bash
cd /workspace/deepSURF/code/harness_generator
export SURF_WORKING_PATH=/workspace/crate_batch_1 \
       GLOBAL_DATA_PATH=/workspace/deepSURF/code/global_data
RUSTFLAGS="-Awarnings" cargo run --release -- "$(ls /workspace/crate_batch_1/deepSURF/report/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
```

LLM-enabled (OpenAI gpt-4o):
```bash
cd /workspace/deepSURF/code/harness_generator
export SURF_WORKING_PATH=/workspace/crate_batch_1 \
       GLOBAL_DATA_PATH=/workspace/deepSURF/code/global_data \
       SURF_ENABLE_LLMS=1 \
       SURF_LLM_PROVIDER=openai \
       LLM_BACKEND=openai/gpt-4o
# Ensure OPENAI_API_KEY is set in this shell
RUSTFLAGS="-Awarnings" cargo run --release -- "$(ls /workspace/crate_batch_1/deepSURF/report/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
```

Outputs land under:
- /workspace/crate_batch_1/deepSURF/fuzz/no_llm/... (static phase)
- /workspace/crate_batch_1/deepSURF/fuzz/llm/... (LLM improvements)

## 4) Notes and tips
- SURF_WORKING_PATH must point to the crate dir (e.g., /workspace/crate_batch_1). The generator reads all inputs/outputs relative to it.
- LLM backend expects the API-style identifier `openai/gpt-4o`; the default remains `deepseek/deepseek-r1` if you leave it unset.
- If you prefer to run without entering the shell, this one-liner also works (we used it successfully):
```bash
docker exec -it deepsurf-work bash -lc '
  set -euo pipefail
  cd /workspace/deepSURF/code/harness_generator
  export SURF_WORKING_PATH=/workspace/crate_batch_1 GLOBAL_DATA_PATH=/workspace/deepSURF/code/global_data SURF_ENABLE_LLMS=1 SURF_LLM_PROVIDER=openai LLM_BACKEND=openai/gpt-4o
  RUSTFLAGS="-Awarnings" cargo run --release -- "$(ls /workspace/crate_batch_1/deepSURF/report/*.urapi.json | head -n1 | sed -E "s/\.urapi\.json$//")"
'
```

- The harness generator respects the workspace-level `.cargo/config.toml` patch we added for `backtrace`; keep that file present so the AFL harness crates build under the rustc_surf toolchain.
- That’s it—repeat steps 2–3 for crate_batch_2..6 by changing SURF_WORKING_PATH.
