#!/usr/bin/env bash
set -euo pipefail

# Number of repetitions for each case (override with: REPEAT=10 ./benchmark.sh)
REPEAT=${REPEAT:-5}

# Resolve binary path (Linux/macOS or Windows .exe)
BIN="./target/release/cubehash"
if [ ! -x "$BIN" ] && [ -x "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi

bench_file() {
  local file="$1"
  local size_label
  size_label="$(basename "$file")"

  for rev in 2 3; do
    local label="CubeHash-${rev} ${size_label}"
    echo "${label} (REPEAT=${REPEAT})"
    for i in $(seq 1 "${REPEAT}"); do
      # Suppress stdout; print timing for each run using bash builtin 'time' in a subshell
      echo -n "  run ${i}: "
      (
        TIMEFORMAT="%3R real, %3U user, %3S sys"
        time "$BIN" -${rev} -l 256 < "${file}" > /dev/null
      ) 2>&1
    done
    echo
  done
}

bench_file testfiles/1M.file
bench_file testfiles/10M.file
bench_file testfiles/100M.file