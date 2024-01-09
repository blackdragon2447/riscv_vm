use clap_num::maybe_hex;
use std::{default, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum GraphicsMode {
    VgaText,
}

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub image: PathBuf,

    #[arg(short = 'H', long, default_value_t = 1)]
    pub hart_count: u8,

    #[arg(short, long)]
    pub graphics_mode: Option<GraphicsMode>,

    #[arg(long, value_parser=maybe_hex::<u64>)]
    pub graphics_address: Option<u64>,

    #[arg(short = 'b', long, default_value_t = true)]
    pub enable_breakpoints: bool,

    #[arg(short, long, default_value_t = false)]
    pub step: bool,
}
