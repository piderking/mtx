help:
    cargo build
    ./target/debug/mtx --help

build-run:
    cargo build
    ./target/debug/mtx "$@"
