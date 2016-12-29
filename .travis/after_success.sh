#!/usr/bin/env sh

set -ex

main() {
    if [ $TARGET = "x86_64-unknown-linux-gnu" ]; then
        travis-cargo --only stable doc
        travis-cargo --only stable doc-upload
    fi
}

main
