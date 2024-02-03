mod memory;

use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
};

use crate::memory::Memory;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let f = BufReader::new(f);

    let mem = Memory::load(f.lines().map(|l| l.unwrap()).collect()).unwrap();

    println!("Loaded memory");

    loop {
        print!("> ");
        let mut buf = String::new();
        stdout().flush().unwrap();
        stdin().read_line(&mut buf).unwrap();
        let buf = buf.strip_suffix('\n').unwrap();

        let args: Vec<&str> = buf.split(' ').collect();
        if let Some(cmd) = args.first() {
            match *cmd {
                "showr" => {
                    let Some(bottom) = args.get(1) else {
                        println!("Missing bottom of range");
                        continue;
                    };

                    let Some(top) = args.get(2) else {
                        println!("Missing top of range");
                        continue;
                    };

                    let Some(bottom) = bottom.strip_prefix("0x") else {
                        println!("Invalid format for bottom of range, use 0xXXXX (hex)");
                        continue;
                    };

                    let Ok(bottom) = usize::from_str_radix(bottom, 16) else {
                        println!("Invalid format for bottom of range, use 0xXXXX (hex)");
                        continue;
                    };

                    let Some(top) = top.strip_prefix("0x") else {
                        println!("Invalid format for top of range, use 0xXXXX (hex)");
                        continue;
                    };

                    let Ok(top) = usize::from_str_radix(top, 16) else {
                        println!("Invalid format for top of range, use 0xXXXX (hex)");
                        continue;
                    };

                    if bottom < (mem.range.start as usize) || bottom > (mem.range.end as usize) {
                        println!("Range bottom out of memory bounds");
                        continue;
                    }

                    if top < (mem.range.start as usize) || top > (mem.range.end as usize) {
                        println!("Range top out of memory bounds");
                        continue;
                    }

                    println!("{:#x}..{:#x}", bottom, top);
                    let range =
                        (bottom - mem.range.start as usize)..(top - mem.range.start as usize);
                    let fragment = &mem.mem[range];

                    print_fragement(fragment);
                }
                "showl" => {
                    let Some(bottom) = args.get(1) else {
                        println!("Missing bottom of range");
                        continue;
                    };

                    let Some(length) = args.get(2) else {
                        println!("Missing length");
                        continue;
                    };

                    let Some(bottom) = bottom.strip_prefix("0x") else {
                        println!("Invalid format for bottom of range, use 0xXXXX (hex)");
                        continue;
                    };

                    let Ok(bottom) = usize::from_str_radix(bottom, 16) else {
                        println!("Invalid format for bottom of range, use 0xXXXX (hex)");
                        continue;
                    };

                    let Ok(length) = length.parse::<usize>() else {
                        println!("Invalid format for bottom of range");
                        continue;
                    };

                    if bottom < (mem.range.start as usize) || bottom > (mem.range.end as usize) {
                        println!("Range bottom out of memory bounds");
                        continue;
                    }

                    if (bottom + length) < (mem.range.start as usize)
                        || (bottom + length) > (mem.range.end as usize)
                    {
                        println!("Range top out of memory bounds");
                        continue;
                    }

                    println!("{:#x}..{:#x}", bottom, bottom + length);
                    let range = (bottom - mem.range.start as usize)
                        ..((bottom + length) - mem.range.start as usize);
                    let fragment = &mem.mem[range];

                    print_fragement(fragment);
                }
                "exit" => break,
                "q" => break,
                _ => println!("Invalid Command"),
            }
        }
    }
}

fn print_fragement(fragment: &[u8]) {
    for line in fragment.chunks(16) {
        for byte in line {
            print!("{:02X} ", byte);
        }
        println!();
    }
}
