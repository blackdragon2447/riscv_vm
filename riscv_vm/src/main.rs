use std::{
    fs,
    io::{stdin, stdout, Write},
    usize,
};

use elf_load::Elf;
#[cfg(feature = "vga_text_buf")]
use riscv_vm::devices::vga_text_mode::VgaTextMode;
use riscv_vm::{devices::simple_uart::SimpleUart, memory::MB, vmstate::VMStateBuilder};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let bytes = fs::read(&args[1]).unwrap();
    let elf = Elf::from_bytes(bytes).unwrap();

    let mut vmstate = VMStateBuilder::<{ 3 * MB }>::default()
        .set_hart_count(1)
        .build();
    vmstate.load_elf_kernel(&elf).unwrap();

    vmstate
        .add_sync_device::<SimpleUart>(0x10000000u64.into())
        .unwrap();

    #[cfg(feature = "vga_text_buf")]
    vmstate
        .add_async_device::<VgaTextMode>(0xB8000u64.into())
        .unwrap();

    // vmstate.step_hart_until(0, 0x2d8u64.into()).unwrap();
    // vmstate.dump_mem();

    'cmdline: loop {
        print!("> ");
        let mut buf = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
        let buf = buf.strip_suffix('\n').unwrap();

        let args: Vec<&str> = buf.split(' ').collect();
        if let Some(cmd) = args.first() {
            match *cmd {
                "step" => {
                    if let Some(count) = args.get(1) {
                        let Ok(count) = count.parse::<usize>() else {
                            println!("Invalid number of steps: {}", count);
                            continue;
                        };
                        for _ in 0..count {
                            if let Err(e) = vmstate.step(false) {
                                println!("Stepping errored at {:?}", e);
                                continue 'cmdline;
                            }
                        }
                    } else {
                        if let Err(e) = vmstate.step(false) {
                            println!("Stepping errored at {:?}", e);
                        }
                    }
                }
                "stepv" => {
                    if let Some(count) = args.get(1) {
                        let Ok(count) = count.parse::<usize>() else {
                            println!("Invalid number of steps: {}", count);
                            continue;
                        };
                        for _ in 0..count {
                            if let Err(e) = vmstate.step(true) {
                                println!("Stepping errored at {:?}", e);
                                continue 'cmdline;
                            }
                        }
                    } else {
                        if let Err(e) = vmstate.step(true) {
                            println!("Stepping errored at {:?}", e);
                        }
                    }
                }
                "step_until" => {
                    if let Some(target) = args.get(1) {
                        match *target {
                            "hart" => {
                                let Some(index) = args.get(2) else {
                                    println!("Missing hart index");
                                    continue;
                                };

                                let Ok(index) = index.parse::<usize>() else {
                                    println!("Invalid hard index: {}", index);
                                    continue;
                                };

                                let Some(target) = args.get(3) else {
                                    println!("Missing target");
                                    continue;
                                };

                                let Some(target) = target.strip_prefix("0x") else {
                                    println!("Invalid format for target, use 0xXXXX (hex)");
                                    continue;
                                };

                                let Ok(target) = u64::from_str_radix(target, 16) else {
                                    println!("Invalid format for target, use 0xXXXX (hex)");
                                    continue;
                                };
                                if let Err(e) = vmstate.step_hart_until(index, target.into()) {
                                    println!("Stepping errored at {:?}", e);
                                }
                            }
                            "all" => {
                                let Some(target) = args.get(2) else {
                                    println!("Missing target");
                                    continue;
                                };

                                let Some(target) = target.strip_prefix("0x") else {
                                    println!("Invalid format for target, use 0xXXXX (hex)");
                                    continue;
                                };

                                let Ok(target) = u64::from_str_radix(target, 16) else {
                                    println!("Invalid format for target, use 0xXXXX (hex)");
                                    continue;
                                };
                                if let Err(e) = vmstate.step_all_until(target.into()) {
                                    println!("Stepping errored at {:?}", e);
                                }
                            }
                            _ => {
                                println!("Invalid target for stepping");
                                continue;
                            }
                        }
                    }
                }
                "state" | "status" => {
                    if let Some(target) = args.get(1) {
                        match *target {
                            "hart" => {
                                let Some(index) = args.get(2) else {
                                    println!("Missing hart index");
                                    continue;
                                };

                                let Ok(index) = index.parse::<usize>() else {
                                    println!("Invalid hard index: {}", index);
                                    continue;
                                };

                                let Some(hart) = vmstate.get_hart(index) else {
                                    println!("Hart with id {} doesn't exist", index);
                                    continue;
                                };

                                println!("{:#?}", hart);
                            }
                            "inst" => {
                                let Some(index) = args.get(2) else {
                                    println!("Missing hart index");
                                    continue;
                                };

                                let Ok(index) = index.parse::<usize>() else {
                                    println!("Invalid hard index: {}", index);
                                    continue;
                                };

                                let Some(_) = vmstate.get_hart(index) else {
                                    println!("Hart with id {} doesn't exist", index);
                                    continue;
                                };

                                match vmstate.fetch(index) {
                                    Ok(inst) => println!("{:#?}", inst),
                                    Err(e) => println!(
                                        "Fetch of instruction for hart {} failed with error{:?}",
                                        index, e
                                    ),
                                }
                            }
                            "pmp" => {
                                let Some(index) = args.get(2) else {
                                    println!("Missing hart index");
                                    continue;
                                };

                                let Ok(index) = index.parse::<usize>() else {
                                    println!("Invalid hard index: {}", index);
                                    continue;
                                };

                                let Some(hart) = vmstate.get_hart(index) else {
                                    println!("Hart with id {} doesn't exist", index);
                                    continue;
                                };

                                println!("{:#?}", hart.get_csr().pmp);
                            }
                            "vmstate" => {
                                println!("{:#?}", vmstate);
                            }
                            // "page_table" => {}
                            _ => {
                                println!("Invalid Subcommand for state")
                            }
                        }
                    } else {
                        println!("{:#?}", vmstate);
                    }
                }
                "dump_mem" => {
                    vmstate.dump_mem();
                    println!("Dumped memory to mem.dump");
                }
                "help" | "h" => {
                    println!("step [count]:");
                    println!("\tIf count is given step all hearts that many cycles");
                    println!("\tOtherwise step all harts once cycle.");
                    println!();
                    println!("stepv [count]:");
                    println!("\tIf count is given step all hearts that many cycles");
                    println!("\tOtherwise step all harts once cycle.");
                    println!("\tPrint the vmstate and instruction on each step.");
                    println!();
                    println!("step_until hart <id> <target>:");
                    println!("step_until all <target>:");
                    println!("\tStep either one hart or all harts until they hit a ");
                    println!("\tgiven address, or they have steped 10000 cycles");
                    println!("\twhichever condition is met first.");
                    println!();
                    println!("state hart <hart_id>:");
                    println!("state pmp <hart_id>:");
                    println!("state inst <hart_id>:");
                    println!("state vmstate:");
                    println!("state:");
                    println!("\tPrint the state of the given hart/pmp, the ");
                    println!("\tentire vm or the current instruction in the");
                    println!("\tgiven hart.");
                    println!();
                    println!("dump_mem:");
                    println!("\tDump the vm's memory to mem.dump for analisys");
                    println!("\tusing meman");
                    println!();
                    println!("run:");
                    println!("\tRun the vm until an mbreak instruction or fatal");
                    println!("\terror is hit. (Option for the vm to request ");
                    println!("\tshutdown will be added).");
                    println!();
                    println!("help:");
                    println!("\tPrint this");
                }
                "exit" | "q" => break,
                _ => println!("Invalid Command"),
            }
        }
    }
}
