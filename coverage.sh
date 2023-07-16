#!/usr/bin/bash

set -e

cargo clean

export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="dxr-%p-%m.profraw"

cargo build --workspace --all-features

cargo test --workspace --all-features
cargo run --all-features --example server > /dev/null &
cargo run --all-features --example client > /dev/null
cargo run --all-features --example koji > /dev/null
kill -INT $!

grcov . -s . --binary-path ./target/debug/ -t html --ignore-not-existing -o ./target/debug/coverage/
rm *.profraw **/*.profraw

