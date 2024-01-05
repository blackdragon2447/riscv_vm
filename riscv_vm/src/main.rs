use std::fs;

use elf_load::{data::ProgramType, ByteRanges, Elf};
use riscv_vm::{
    decode::decode,
    memory::{self, Address, Memory, MemoryError},
};

fn main() {
    let bytes = fs::read("./test_os/os.elf").unwrap();
    let mut elf = Elf::from_bytes(bytes).unwrap();
    elf.populate_secion_names().unwrap();

    dbg!(&elf.header);
    dbg!(&elf.program_headers);
    dbg!(&elf.section_headers);

    let mut mem = Memory::<{ 16 * memory::KB }>::new();

    let addr = load_elf_phys(&elf, &mut mem).unwrap();

    dbg!(&addr);
    let mut buf: [u8; 4] = [0; 4];
    buf.copy_from_slice(mem.read_bytes(addr, 4).unwrap());
    dbg!(decode(u32::from_le_bytes(buf)));
}

pub fn load_elf_phys<const SIZE: usize>(
    elf: &Elf,
    mem: &mut Memory<SIZE>,
) -> Result<Address, MemoryError> {
    for h in &elf.program_headers {
        if h.program_type == ProgramType::Load && h.seg_m_size.0 != 0 {
            let bytes = elf.bytes.get_bytes(h.seg_offset, h.seg_f_size.0);
            mem.write_bytes(bytes, h.seg_p_addr.into())?;
        }
    }

    Ok(elf.header.entry.into())
}
