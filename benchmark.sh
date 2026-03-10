#!/usr/bin/env bash
set -euo pipefail

# ── Configuration ─────────────────────────────────────────────────────────────
# Number of repetitions per benchmark case  (override: REPEAT=10 ./benchmark.sh)
REPEAT=${REPEAT:-5}
# Output CSV file                          (override: OUTPUT_CSV=out.csv ./benchmark.sh)
OUTPUT_CSV="${OUTPUT_CSV:-benchmark_results.csv}"
# Space-separated CubeHash revisions to test
REVISIONS="${REVISIONS:-2 3}"
# Space-separated hash lengths in bits to test (must be ≤512 and divisible by 8)
HASH_BITS="${HASH_BITS:-256}"
# Space-separated test files
TEST_FILES="${TEST_FILES:-testfiles/1M.file testfiles/10M.file testfiles/100M.file testfiles/256M.file testfiles/512M.file testfiles/1G.file testfiles/2G.file}"
# ──────────────────────────────────────────────────────────────────────────────

# Resolve binary path (Linux/macOS or Windows .exe)
BIN="./target/release/cubehash"
if [ ! -x "$BIN" ] && [ -x "${BIN}.exe" ]; then
  BIN="${BIN}.exe"
fi

if [ ! -x "$BIN" ]; then
  echo "Error: binary not found at $BIN (run 'cargo build --release' first)" >&2
  exit 1
fi

TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

# Write CSV header
echo "timestamp,file,size_bytes,rev,hash_bits,run,real_s,user_s,sys_s,throughput_mbs" \
  > "$OUTPUT_CSV"

# ── Helper: compute stats for an array of numbers ─────────────────────────────
# Usage: stats=$(compute_stats val1 val2 ...)
# Outputs: "avg min max" on one line
compute_stats() {
  printf '%s\n' "$@" | awk '
    { s += $1; if (NR == 1 || $1 < mn) mn = $1; if (NR == 1 || $1 > mx) mx = $1 }
    END { printf "%.6f %.6f %.6f", s/NR, mn, mx }
  '
}

# ── Benchmark one (file × revision × hash_bits) combination ───────────────────
bench_case() {
  local file="$1" rev="$2" bits="$3"
  local size_label
  size_label="$(basename "$file")"

  # File size in bytes for throughput calculation
  local size_bytes
  size_bytes=$(wc -c < "$file" | awk '{print $1}')

  local label="CubeHash${rev}-${bits} ${size_label}"
  printf '%s (REPEAT=%d)\n' "$label" "$REPEAT"

  local reals=() users=() syss=()

  for i in $(seq 1 "${REPEAT}"); do
    printf '  run %d: ' "$i"

    local timing real user sys throughput
    timing=$(
      TIMEFORMAT="%3R %3U %3S"
      { time "$BIN" -${rev} -l "${bits}" < "${file}" > /dev/null; } 2>&1
    )

    read -r real user sys <<< "$timing"

    # Throughput in MB/s  (size / real_seconds / 1048576)
    throughput=$(awk -v s="$size_bytes" -v t="$real" \
      'BEGIN { if (t > 0) printf "%.2f", s / t / 1048576; else print "inf" }')

    printf '%ss real, %ss user, %ss sys  (%s MB/s)\n' \
      "$real" "$user" "$sys" "$throughput"

    reals+=("$real")
    users+=("$user")
    syss+=("$sys")

    # Append raw-run row to CSV
    echo "${TIMESTAMP},${size_label},${size_bytes},${rev},${bits},${i},${real},${user},${sys},${throughput}" \
      >> "$OUTPUT_CSV"
  done

  # Compute summary stats
  local r_stats u_stats s_stats
  r_stats=$(compute_stats "${reals[@]}")
  u_stats=$(compute_stats "${users[@]}")
  s_stats=$(compute_stats "${syss[@]}")

  local avg_r min_r max_r avg_u avg_s avg_tp
  read -r avg_r min_r max_r <<< "$r_stats"
  avg_u=$(awk '{print $1}' <<< "$u_stats")
  avg_s=$(awk '{print $1}' <<< "$s_stats")

  avg_tp=$(awk -v s="$size_bytes" -v t="$avg_r" \
    'BEGIN { if (t > 0) printf "%.2f", s / t / 1048576; else print "inf" }')

  printf '  --- avg: %ss real, %ss user, %ss sys  (%s MB/s)\n' \
    "$avg_r" "$avg_u" "$avg_s" "$avg_tp"
  printf '      min: %ss  max: %ss\n\n' "$min_r" "$max_r"

  # Append summary rows to CSV
  echo "${TIMESTAMP},${size_label},${size_bytes},${rev},${bits},avg,${avg_r},${avg_u},${avg_s},${avg_tp}" \
    >> "$OUTPUT_CSV"
  echo "${TIMESTAMP},${size_label},${size_bytes},${rev},${bits},min,${min_r},,,"\
    >> "$OUTPUT_CSV"
  echo "${TIMESTAMP},${size_label},${size_bytes},${rev},${bits},max,${max_r},,,"\
    >> "$OUTPUT_CSV"
}

# ── Run all combinations ───────────────────────────────────────────────────────
for file in $TEST_FILES; do
  if [ ! -f "$file" ]; then
    echo "Warning: test file '$file' not found, skipping." >&2
    continue
  fi
  for rev in $REVISIONS; do
    for bits in $HASH_BITS; do
      bench_case "$file" "$rev" "$bits"
    done
  done
done

echo "Results written to: $OUTPUT_CSV"
