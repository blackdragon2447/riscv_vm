use std::fs;

use riscv_vm::elf_load::{
    elf_header::ElfHeader, program_header::ProgramHeader, section_header::SectionHeader,
};

fn main() {
    let bytes = fs::read("./test_os/os.elf").unwrap();
    let header = ElfHeader::from_bytes(&bytes).unwrap();

    let mut p_headers = vec![];

    for i in 0..header.p_header_ecount {
        p_headers.push(
            ProgramHeader::from_bytes(
                &bytes,
                header.p_header + (i * header.p_header_size) as u64,
                header.p_header_size.into(),
            ), // .unwrap(),
        );
    }

    let mut s_headers = vec![];

    for i in 0..header.s_header_ecount {
        s_headers.push(SectionHeader::from_bytes(
            &bytes,
            header.s_header + (i * header.s_header_size) as u64,
            header.s_header_size.into(),
        ))
    }

    dbg!(header);
    dbg!(p_headers);
    dbg!(s_headers);
}
