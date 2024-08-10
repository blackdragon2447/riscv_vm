# riscv_vm: Virtualizing RISC-V

My (not so) small experiment with implementing a RISC-V vm/emulator in rust.

## Running

Running can be done through cargo, using `cargo run --bin riscv_vm -- [ARGS]`
or through just, with `just run [KERNEL]`. Running through just doesn't 
allow for passing additional args, past the kernel file to load. For 
commandline arguments, and their desctiptions, see 
`cargo run --bin riscv_vm -- --help`.

When running, the minimum that is required is a kernel, this kernel is 
expected to be an elf file, compiled for rv64, that should be loaded into 
memory at `0x80000000` and has its entry point at that address.

## Building

Building is generally straightforward, requiring just a `cargo build` or 
`just build`, or their release equivalents. The exeption to this is the 
`float` feature, which is enabled by default and provides support for the 
`F` and `D`, extensions. The `float` feature requires CMake to be installed 
to be able to compile a soft float library.

Available features are:

- `vga_text_buffer`: provide a default vga text buffer to the vm (currently broken).
- `float`: provide support for the `F` and `D`, extensions.

## Testing

Testing of certain features needs binaries built from the assembly find in 
`vm_tests`, the easiest way to build these binaries is using `just setup-tests`,
here the `RISCV_PREFIX` can be used to supply a custom toolchain to the builder
of the official tests, and `RISCV_CC` to provide a custom compiler to the custom 
tests.
