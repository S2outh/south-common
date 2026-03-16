#!/usr/bin/env bash
set -euo pipefail

cargo rustdoc -- -Z unstable-options --output-format json

INPUT="target/doc/south_common.json"

# Iterate safely over JSON array using jq
jq -c '.index | to_entries[] | select(.value.name=="__TOOLING_METADATA") | .value.inner.constant.const.expr' "$INPUT" \
| while IFS= read -r val; do
    # Increment counter
    i=${i:-0}
    i=$((i+1))

    # val is a JSON string literal, unescape it
    inner=$(echo "$val" | jq -r '.' | jq -r '.')

    # Pretty-print and save
    echo "$inner" | jq '.' > "tooling/export_${i}.json"
done
