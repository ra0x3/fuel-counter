#!/bin/bash
set -vXo
# https://github.com/rust-rocksdb/rust-rocksdb/issues/535
RUSTFLAGS="-Clink-arg=-Wl,--allow-multiple-definition" cargo build
RUST_LOG=debug ./target/debug/server
