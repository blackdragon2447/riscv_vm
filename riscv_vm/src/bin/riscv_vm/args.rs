use std::{convert::Infallible, fmt::Display, path::PathBuf, str::FromStr};

use riscv_vm::vmstate::VMSettings;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct VMArgs {
    pub settings: VMSettings,
    pub hart_count: u64,
    pub kernel: Option<PathBuf>,
    // firmware: Option<PathBuf>,
    pub graphic: bool,
    pub uart: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseArgError {
    DoubleArg(String),
    InvalidHex(String),
    MissingValue(String),
    UnknownArgument(String),
    Other(String),
    PrintHelp,
}

impl Display for ParseArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseArgError::DoubleArg(arg) => writeln!(f, "Argument {arg} (or its negation) is provided multiple times,\nplease this argument only once."),
            ParseArgError::InvalidHex(hex) => writeln!(f, "\"{hex}\" is not a valid hex value,\nhex values must start with '0x' and only contain [a-fA-F0-9]."),
            ParseArgError::MissingValue(arg) => writeln!(f, "Argument \"{arg}\" requires a value, which is not provided"),
            ParseArgError::UnknownArgument(arg) => writeln!(f, "\"{arg}\" is not a valid argument, see --help for valid arguments"),
            ParseArgError::Other(txt) => writeln!(f, "{}", txt),
            ParseArgError::PrintHelp => Ok(print_help()),
        }
    }
}

pub fn parse_args<I: Iterator<Item = String>>(mut args: I) -> Result<VMArgs, ParseArgError> {
    let mut pmp_enable = None;
    let mut virt_mem_enable = None;
    let mut timer_addr = None;

    let mut m_mode_swi_enable = None;
    let mut m_mode_swi_addr = None;

    let mut s_mode_swi_enable = None;
    let mut s_mode_swi_addr = None;

    let mut reset_vec = None;

    let mut hart_count = None;

    let mut kernel = None;
    // let mut firmware = None;
    let mut graphic = None;
    let mut uart = None;

    let _ = args.next(); // args[0] is the bin name

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--pmp" => try_set_arg(&mut pmp_enable, true, "--pmp".to_string())?,
            "--no-pmp" => try_set_arg(&mut pmp_enable, false, "--no-pmp".to_string())?,

            "--virt-mem" => try_set_arg(&mut virt_mem_enable, true, "--virt-mem".to_string())?,
            "--no-virt-mem" => {
                try_set_arg(&mut virt_mem_enable, false, "--no-virt-mem".to_string())?
            }

            "--timer-address" => try_set_arg(
                &mut timer_addr,
                parse_hex(
                    args.next()
                        .ok_or(ParseArgError::MissingValue("--timer-address".to_string()))?,
                )?,
                "--timer-address".to_string(),
            )?,

            "--mswi" => try_set_arg(&mut m_mode_swi_enable, true, "--mswi".to_string())?,
            "--no-mswi" => try_set_arg(&mut m_mode_swi_enable, false, "--no-mswi".to_string())?,
            "--mswi-address" => try_set_arg(
                &mut m_mode_swi_addr,
                parse_hex(
                    args.next()
                        .ok_or(ParseArgError::MissingValue("--mswi-address".to_string()))?,
                )?,
                "--mswi-address".to_string(),
            )?,

            "--sswi" => try_set_arg(&mut s_mode_swi_enable, true, "--sswi".to_string())?,
            "--no-sswi" => try_set_arg(&mut s_mode_swi_enable, false, "--no-sswi".to_string())?,
            "--sswi-address" => try_set_arg(
                &mut s_mode_swi_addr,
                parse_hex(
                    args.next()
                        .ok_or(ParseArgError::MissingValue("--sswi-address".to_string()))?,
                )?,
                "--sswi-address".to_string(),
            )?,

            "--reset-vec" => try_set_arg(
                &mut reset_vec,
                parse_hex(
                    args.next()
                        .map(|s| if s.starts_with("--") { None } else { Some(s) })
                        .flatten()
                        .ok_or(ParseArgError::MissingValue("--reset-vec".to_string()))?,
                )?,
                "--reset-vec".to_string(),
            )?,

            "--harts" => try_set_arg(
                &mut hart_count,
                args.next()
                    .map(|s| if s.starts_with("--") { None } else { Some(s) })
                    .flatten()
                    .ok_or(ParseArgError::MissingValue("--reset-vec".to_string()))?
                    .parse::<u64>()
                    .map_err(|_| {
                        ParseArgError::Other(
                            "Please provide a positive integer for the number of harts".to_string(),
                        )
                    })?,
                "--reset-vec".to_string(),
            )?,

            "--kernel" => try_set_arg(
                &mut kernel,
                PathBuf::from_str(
                    &args
                        .next()
                        .ok_or(ParseArgError::MissingValue("--kernel".to_string()))?,
                )?,
                "--kernel".to_string(),
            )?,

            "--graphic" => try_set_arg(&mut graphic, true, "--graphic".to_string())?,
            "--no-graphic" => try_set_arg(&mut graphic, false, "--no-graphic".to_string())?,

            "--uart" => try_set_arg(&mut uart, true, "--uart".to_string())?,
            "--no-uart" => try_set_arg(&mut uart, false, "--no-uart".to_string())?,

            "--help" => return Err(ParseArgError::PrintHelp),
            s => return Err(ParseArgError::UnknownArgument(s.to_string())),
        }
    }

    let mut vm_args = VMArgs::default();
    let settings = &mut vm_args.settings;

    if let Some(pmp_enable) = pmp_enable {
        settings.pmp_enable = pmp_enable;
    }

    if let Some(timer_addr) = timer_addr {
        settings.timer_addr = timer_addr.into();
    }

    if let Some(virt_mem_enable) = virt_mem_enable {
        settings.virt_mem_enable = virt_mem_enable;
    }

    if let Some(s_mode_swi_enable) = s_mode_swi_enable {
        settings.s_mode_swi_enable = s_mode_swi_enable;
    }
    if let Some(s_mode_swi_addr) = s_mode_swi_addr {
        settings.s_mode_swi_addr = s_mode_swi_addr.into();
    }

    if let Some(m_mode_swi_enable) = m_mode_swi_enable {
        settings.m_mode_swi_enable = m_mode_swi_enable;
    }
    if let Some(m_mode_swi_addr) = m_mode_swi_addr {
        settings.m_mode_swi_addr = m_mode_swi_addr.into();
    }

    if let Some(reset_vec) = reset_vec {
        settings.reset_vec = reset_vec.into();
    }

    if let Some(hart_count) = hart_count {
        vm_args.hart_count = hart_count;
    }

    vm_args.kernel = kernel;
    if vm_args.kernel.is_none() {
        return Err(ParseArgError::Other(
            "Please specify a kernel, as no kernel file currently results in an unbootable vm"
                .to_string(),
        ));
    }

    if let Some(graphic) = graphic {
        if !cfg!(feature = "vga_text_buf") {
            return Err(ParseArgError::Other(
                "Graphic output is unsupported, see --help for more".to_string(),
            ));
        }
        vm_args.graphic = graphic;
    }

    if let Some(uart) = uart {
        vm_args.uart = uart;
    }

    Ok(vm_args)
}

pub fn print_help() {
    println!(include_str!("./help.txt"));
}

fn parse_hex(hex: String) -> Result<u64, ParseArgError> {
    let Some(value) = hex.strip_prefix("0x") else {
        return Err(ParseArgError::InvalidHex(hex));
    };

    let Ok(value) = u64::from_str_radix(value, 16) else {
        return Err(ParseArgError::InvalidHex(hex));
    };

    Ok(value)
}

fn try_set_arg<T>(arg: &mut Option<T>, value: T, name: String) -> Result<(), ParseArgError> {
    if arg.is_none() {
        *arg = Some(value);
        Ok(())
    } else {
        return Err(ParseArgError::DoubleArg(name));
    }
}

impl From<Infallible> for ParseArgError {
    fn from(_value: Infallible) -> Self {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use riscv_vm::vmstate::VMSettings;

    use crate::args::{parse_args, ParseArgError, VMArgs};

    #[test]
    fn help() {
        assert_eq!(
            parse_args(vec!["--help".to_string()].into_iter()),
            Err(ParseArgError::PrintHelp)
        );
    }

    #[test]
    fn normal_args() {
        assert_eq!(
            parse_args(vec!["--kernel".to_string(), "/dev/null".to_string()].into_iter()),
            Ok(VMArgs {
                kernel: Some(PathBuf::from("/dev/null")),
                ..Default::default()
            })
        );
        assert_eq!(
            parse_args(
                vec![
                    "--kernel".to_string(),
                    "/dev/null".to_string(),
                    "--graphic".to_string()
                ]
                .into_iter()
            ),
            Ok(VMArgs {
                kernel: Some(PathBuf::from("/dev/null")),
                graphic: true,
                ..Default::default()
            })
        );
        assert_eq!(
            parse_args(
                vec![
                    "--kernel".to_string(),
                    "/dev/null".to_string(),
                    "--no-graphic".to_string()
                ]
                .into_iter()
            ),
            Ok(VMArgs {
                kernel: Some(PathBuf::from("/dev/null")),
                graphic: false,
                ..Default::default()
            })
        );
        assert_eq!(
            parse_args(
                vec![
                    "--kernel".to_string(),
                    "/dev/null".to_string(),
                    "--reset-vec".to_string(),
                    "0x70000000".to_string()
                ]
                .into_iter()
            ),
            Ok(VMArgs {
                kernel: Some(PathBuf::from("/dev/null")),
                settings: VMSettings {
                    reset_vec: 0x70000000u64.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
        );
    }
}
