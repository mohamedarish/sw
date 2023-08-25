#!/usr/bin/sh

cargo fmt --check --all || exit 1
cargo clippy || exit 1
cargo test || exit 1
