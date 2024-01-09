use clap::Parser;
use riscv_vm::{args::Args, gui};

fn main() {
    let args = Args::parse();

    dbg!(&args);

    gui::run(args).unwrap();

    // loop {
    // vmstate.step().unwrap();
    // dbg!(&vmstate);
    // let mut buf = String::new();
    // stdin().read_line(&mut buf).unwrap();
    // }
}
