#!/bin/bash
# shellcheck disable=SC2091
# shellcheck disable=SC2164
cd "$(dirname "$0")"

cargo watch -w src -w Cargo.toml -x run
