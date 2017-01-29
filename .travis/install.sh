#!/usr/bin/env sh

set -ex

host() {
    case $TRAVIS_OS_NAME in
        linux)
            echo x86_64-unknown-linux-gnu
            ;;
        osx)
            echo x86_64-apple-darwin
            ;;
    esac
}

gcc_prefix() {
    case $TARGET in
        *-musl)
            echo musl
            ;;
        *)
            return
            ;;
    esac
}

install_rustup() {
    sh ~/rust/lib/rustlib/uninstall.sh

    curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain=$TRAVIS_RUST_VERSION

    rustc -Vv
    cargo -V
}

install_toolchain() {
    if [ $(host) != $TARGET ]; then
        rustup target add $TARGET
    fi
}

configure_linker() {
    local prefix=$(gcc_prefix)

    if [ ! -z $prefix ]; then
        ${prefix}-gcc -v

        mkdir -p .cargo
        cat >> .cargo/config << EOF
[target.$TARGET]
linker = "${prefix}-gcc"
EOF
    fi
}

main() {
    install_rustup
    install_toolchain
    configure_linker
}

main
