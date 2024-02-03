use std::{num::ParseIntError, ops::Range};

pub struct Memory {
    pub mem: Vec<u8>,
    pub range: Range<u64>,
}

#[derive(Debug)]
pub enum LoadError {
    EmptyFile,
    RangeMissing,
    InvalidByte,
}

impl Memory {
    pub fn load(lines: Vec<String>) -> Result<Self, LoadError> {
        if lines.len() < 2 {
            return Err(LoadError::EmptyFile);
        }

        let range_line: Vec<&str> = lines[0].split(": ").collect();
        if range_line[0] != "range" {
            return Err(LoadError::RangeMissing);
        }

        let range: Vec<u64> = range_line[1]
            .split("..")
            .filter_map(|s| s.strip_prefix("0x"))
            .filter_map(|s| u64::from_str_radix(s, 16).ok())
            .collect();

        if range.len() != 2 {
            return Err(LoadError::RangeMissing);
        }

        let range = range[0]..range[1];

        let mut bytes = Vec::new();
        for line in &lines[1..] {
            for byte in line.split(' ').filter(|b| b != &"") {
                bytes.push(u8::from_str_radix(byte, 16)?);
            }
        }

        Ok(Self { mem: bytes, range })
    }
}

impl From<ParseIntError> for LoadError {
    fn from(_value: ParseIntError) -> Self {
        Self::InvalidByte
    }
}
