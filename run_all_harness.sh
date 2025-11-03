#!/usr/bin/env bash
#
# Run every AFL harness produced by deepSURF for a given crate batch.
# Usage: ./run_all_harness.sh <project_root>
# Example: ./run_all_harness.sh crate_batch_1
#
# The script expects the following layout (which is what deepSURF emits):
#   <project_root>/deepSURF/fuzz/fuzzing_corpus/<harness_dir>/
# Each harness directory must contain:
#   bins/<target_name>_non_asan (instrumented binary produced by cargo-afl)
#   input/                      (seed corpus; created automatically if empty)
#
# For every harness, we spawn a tmux window in a session named
#   "${project_name}fuzz"
# and run `cargo afl fuzz` in-place.  All windows share the same session so you
# can monitor / stop the fuzzers by attaching to that tmux session afterwards.

set -Eeuo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <project_root>" >&2
  exit 2
fi

if ! command -v tmux >/dev/null 2>&1; then
  echo "[run_all_harness] ERROR: tmux is required but not found in PATH." >&2
  exit 2
fi
if ! command -v cargo >/dev/null 2>&1; then
  echo "[run_all_harness] ERROR: cargo is required but not found in PATH." >&2
  exit 2
fi

PROJECT_ROOT="$1"
if [[ ! -d "$PROJECT_ROOT" ]]; then
  echo "[run_all_harness] ERROR: project root not found: $PROJECT_ROOT" >&2
  exit 2
fi

# Resolve to absolute path to avoid surprises once tmux windows cd.
PROJECT_ROOT="$(readlink -f "$PROJECT_ROOT")"
PROJECT_NAME="$(basename "$PROJECT_ROOT")"
FUZZ_ROOT="$PROJECT_ROOT/deepSURF/fuzz/fuzzing_corpus"

if [[ ! -d "$FUZZ_ROOT" ]]; then
  echo "[run_all_harness] ERROR: fuzzing corpus directory missing: $FUZZ_ROOT" >&2
  exit 2
fi

# Collect harness directories (only immediate children).
mapfile -t HARNESS_DIRS < <(find "$FUZZ_ROOT" -mindepth 1 -maxdepth 1 -type d | sort)
if [[ ${#HARNESS_DIRS[@]} -eq 0 ]]; then
  echo "[run_all_harness] No harnesses found under $FUZZ_ROOT" >&2
  exit 0
fi

SESSION_NAME="${PROJECT_NAME}fuzz"
if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
  echo "[run_all_harness] ERROR: tmux session '$SESSION_NAME' already exists." >&2
  echo "Please terminate it first (tmux kill-session -t $SESSION_NAME)." >&2
  exit 2
fi

# Allow overriding the toolchain via env (defaults to rustc-fuzzing installed by deepSURF).
DEFAULT_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-rustc-fuzzing}"
ACTIVE_TOOLCHAIN="$(rustup show active-toolchain 2>/dev/null | awk '{print $1}' || true)"
TOOLCHAIN_FLAG=""

normalize_toolchain_list() {
  rustup toolchain list 2>/dev/null | awk '{print $1}'
}

if [[ -n "$DEFAULT_TOOLCHAIN" ]]; then
  if normalize_toolchain_list | grep -Fxq "$DEFAULT_TOOLCHAIN"; then
    TOOLCHAIN_FLAG="+$DEFAULT_TOOLCHAIN"
  else
    echo "[run_all_harness] WARN: rustup toolchain '$DEFAULT_TOOLCHAIN' not installed; falling back to active toolchain." >&2
  fi
fi

if [[ -z "$TOOLCHAIN_FLAG" && -n "$ACTIVE_TOOLCHAIN" ]]; then
  TOOLCHAIN_FLAG="+$ACTIVE_TOOLCHAIN"
fi

if [[ -n "$TOOLCHAIN_FLAG" ]]; then
  if ! cargo $TOOLCHAIN_FLAG --version >/dev/null 2>&1; then
    echo "[run_all_harness] WARN: toolchain '$TOOLCHAIN_FLAG' unavailable; using default cargo invocation." >&2
    TOOLCHAIN_FLAG=""
  fi
fi

# Helper to create a minimal seed if the input corpus is empty.
ensure_seed() {
  local dir="$1"
  mkdir -p "$dir"
  if [[ -z "$(ls -A "$dir" 2>/dev/null)" ]]; then
    printf '\0' >"$dir/seed0"
  fi
}

# Sanitize window names for tmux.
sanitize_name() {
  local raw="$1"
  raw="${raw%%_deepseek-r1_turn*}" # trim very long suffixes seen in deepSURF outputs
  raw="${raw%%_gpt-4o_turn*}"
  raw="${raw//[^[:alnum:]_-]/_}"
  printf "%.*s" 30 "$raw"          # tmux window name limit; keep short and readable
}

# Build tmux command string for each harness.
build_cmd() {
  local harness="$1"
  local bin_non
  bin_non="$(find "$harness/bins" -maxdepth 1 -type f -name '*_non_asan' | head -n1 || true)"
  if [[ -z "$bin_non" ]]; then
    echo ""
    return
  fi

  ensure_seed "$harness/input"
  rm -rf "$harness/out"
  mkdir -p "$harness/out"

  cat <<EOF
cd "$harness"
echo "[run_all_harness] fuzzing $(basename "$harness")"
exec cargo $TOOLCHAIN_FLAG afl fuzz -i input -o out -- "$bin_non"
EOF
}

FIRST=1
for harness in "${HARNESS_DIRS[@]}"; do
  base="$(basename "$harness")"
  cmd="$(build_cmd "$harness")"
  if [[ -z "$cmd" ]]; then
    echo "[run_all_harness] WARN: skipping $base (no *_non_asan binary found)." >&2
    continue
  fi

  window_name="$(sanitize_name "$base")"
  if [[ $FIRST -eq 1 ]]; then
    tmux new-session -d -s "$SESSION_NAME" -n "$window_name" "bash -lc $(printf '%q' "$cmd")"
    FIRST=0
  else
    tmux new-window -t "$SESSION_NAME" -n "$window_name" "bash -lc $(printf '%q' "$cmd")"
  fi
  echo "[run_all_harness] Launched tmux window '$window_name' for $base"
done

if [[ $FIRST -eq 1 ]]; then
  # We never launched anything.
  echo "[run_all_harness] No runnable harnesses discovered under $FUZZ_ROOT" >&2
  tmux kill-session -t "$SESSION_NAME" >/dev/null 2>&1 || true
  exit 0
fi

FUZZ_DURATION_SECONDS="${FUZZ_DURATION_SECONDS:-86400}"
if [[ "$FUZZ_DURATION_SECONDS" =~ ^[0-9]+$ && "$FUZZ_DURATION_SECONDS" -gt 0 ]]; then
  tmux new-window -t "$SESSION_NAME" -n "auto-stop" \
    "bash -lc 'echo \"[run_all_harness] auto-stop in ${FUZZ_DURATION_SECONDS}s\"; sleep $FUZZ_DURATION_SECONDS; tmux list-sessions >/dev/null 2>&1 || exit 0; echo \"[run_all_harness] Stopping fuzzers after ${FUZZ_DURATION_SECONDS}s\"; tmux kill-session -t $SESSION_NAME'"
  echo "[run_all_harness] Auto-stop window scheduled after ${FUZZ_DURATION_SECONDS}s."
fi

echo "[run_all_harness] tmux session '$SESSION_NAME' is ready."
echo "Attach with: tmux attach -t $SESSION_NAME"
