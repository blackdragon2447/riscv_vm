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

clippy:
    cargo clippy -- \
      -W clippy::allow_attributes_without_reason \
      -W clippy::dbg_macro \
      -W clippy::decimal_literal_representation \
      -W clippy::deref_by_slicing \
      -W clippy::expect_used \
      -W clippy::get_unwrap \
      -W clippy::if_then_some_else_none \
      -W clippy::let_underscore_must_use \
      -W clippy::mutex_atomic \
      -W clippy::rc_buffer \
      -W clippy::rc_mutex \
      -W clippy::same_name_method \
      -W clippy::self_named_module_files \
      -W clippy::suspicious_xor_used_as_pow \
      -W clippy::tests_outside_test_module \
      -W clippy::todo \
      -W clippy::unnecessary_self_imports

working: build test clippy
