# riscv_vm: Virtualizing RISC-V

My small experiment with implementing a RISC-V vm in rust.

## Running

To run it,
just do `cargo run -- <elf_file>`, the file must be of elf format,
and it must specify that the first instruction is at `0x80000000`
(start of ram), any character that is not `null` written to
`0x10000000` is outputted to the terminal. If you enable the
`vga_text buf` feature a standard vga text buffer is available
at `0xB8000`. The vm is currently set to 4 KB of ram, if you
need nore, in `riscv_vm/src/main.rs` change the `{ 4 * KB }`
to something else (leave the curly braces there). If you want
to specify memory in MB inport `riscv_vm::memory::MB`. If you
want to step instruction by instruction, uncomment the code in
the loop in `main()` (in `main.rs`) and `//println!("{:#?}", &inst);`
at `riscv_vm/src/hart/mod.rs:94`. That way It'll print the state
of the vm before running the printed instruction and runs the
instruction.

The justfile is partially outdated, it can be used for building
the tests and testing but not for running.

## Testing

The tests conists of two parts, the code tests, and the vm tests,
the code tests don't need any setup and can just be run with a simple
`cargo test` or `just test`, the vm tests need to be built first,
to do so, make sure that all submodules are initialized, then run
`just setup-tests`. If your riscv toolchain has a prefix diffirent than
`riscv64-unknown-elf-` (it is installed as `riscv64-elf-` or isn't in
$PATH and needs a path in front), make sure that the `RISCV_PREFIX` env
variable is set to the desired prefix.
