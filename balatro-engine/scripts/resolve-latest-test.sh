#!/bin/bash
set -euo pipefail
cargo test --no-run --message-format=json \
  | jq -r 'select(.profile.test==true and (.target.kind|tostring|contains("lib"))) | .executable' \
  | tail -n1

