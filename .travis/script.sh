#!/usr/bin/env sh

set -ex

main() {
    cargo build --target $TARGET --verbose
    # cargo test --target $TARGET
}

main
