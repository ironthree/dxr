#!/usr/bin/bash

set -e

export RUSTFLAGS="-Cinstrument-coverage"
cargo build --workspace --all-features
export LLVM_PROFILE_FILE="dxr-%p-%m.profraw"
cargo test --workspace --all-features
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
rm *.profraw **/*.profraw

