build:
    cargo build

build-release:
    cargo build --release

run:
    cargo run

setup-tests:
    cd ./riscv-tests/ && autoconf
    cd ./riscv-tests/ && ./configure
    cd ./riscv-tests/ && make isa

test: build
    cargo test
