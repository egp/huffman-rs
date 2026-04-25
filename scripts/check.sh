#!/bin/zsh

cargo fmt
cargo clippy -- -D warnings
cargo check
cargo test
