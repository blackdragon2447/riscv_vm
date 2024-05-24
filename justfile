build:
    cargo build

build-release:
    cargo build --release

run binary:
	cargo run --bin riscv_vm -- {{binary}}

docs:
	cargo doc --no-deps
	rm -rf ./docs
	echo "<meta http-equiv=\"refresh\" content=\"0; url=riscv_vm\">" > target/doc/index.html
	cp -r target/doc ./docs

setup-tests:
    cd ./vm_tests/official_tests/ && autoconf
    cd ./vm_tests/official_tests/ && ./configure
    -cd ./vm_tests/official_tests/ && make
    cd ./vm_tests/official_tests/ && make isa
    cd ./vm_tests/custom_tests/ && just build

clean:
    cargo clean
    cd ./vm_tests/official_tests/ && make clean
    cd ./vm_tests/custom_tests/ && just clean

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
      -W clippy::unnecessary_self_imports \
      -D unused_must_use

check: build test clippy
