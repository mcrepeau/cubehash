#!/usr/bin/env bash

set -euo pipefail

MANIFEST="testfiles/manifest.txt"
if [[ ! -f "$MANIFEST" ]]; then
  echo "No manifest found at $MANIFEST. Skipping verification on native." >&2
  exit 0
fi

# Format: path,revision,hashlen,expected_hex
# revision: 2 or 3
# hashlen: e.g. 256
fail=0
while IFS=, read -r path rev hashlen expected; do
  [[ -z "$path" || "$path" =~ ^\s*# ]] && continue
  path=$(echo "$path" | xargs)
  rev=$(echo "$rev" | xargs)
  hashlen=$(echo "$hashlen" | xargs)
  expected=$(echo "$expected" | tr -d '\r\n' | xargs)
  if [[ ! -f "$path" ]]; then
    echo "Missing test file: $path" >&2
    fail=1
    continue
  fi
  revflag="-3"
  [[ "$rev" == "2" ]] && revflag="-2"
  out=$(cargo run --release --quiet -- $revflag -l "$hashlen" < "$path")
  if [[ "$out" != "$expected" ]]; then
    echo "Mismatch: $path (rev=$rev len=$hashlen)" >&2
    echo " expected: $expected" >&2
    echo "   actual: $out" >&2
    fail=1
  else
    echo "OK: $path (rev=$rev len=$hashlen)"
  fi
done < "$MANIFEST"
exit $fail


