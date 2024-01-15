use std::{fs, process::exit};

use elf_load::Elf;

use crate::{memory::KB, vmstate::VMState};

#[macro_use]
mod util;

isa_test!(rv64ui_p_add);
isa_test!(rv64ui_p_addi);
isa_test!(rv64ui_p_addiw);
isa_test!(rv64ui_p_addw);
isa_test!(rv64ui_p_and);
isa_test!(rv64ui_p_andi);
isa_test!(rv64ui_p_auipc);
isa_test!(rv64ui_p_beq);
isa_test!(rv64ui_p_bge);
isa_test!(rv64ui_p_bgeu);
isa_test!(rv64ui_p_blt);
isa_test!(rv64ui_p_bltu);
isa_test!(rv64ui_p_bne);
// isa_test!(rv64ui_p_fence_i);
isa_test!(rv64ui_p_jal);
isa_test!(rv64ui_p_jalr);
isa_test!(rv64ui_p_lb, { 16 * KB });
isa_test!(rv64ui_p_lbu, { 16 * KB });
isa_test!(rv64ui_p_ld, { 16 * KB });
isa_test!(rv64ui_p_lh, { 16 * KB });
isa_test!(rv64ui_p_lhu, { 16 * KB });
isa_test!(rv64ui_p_lui);
isa_test!(rv64ui_p_lwu, { 16 * KB });
// isa_test!(rv64ui_p_ma_data);
isa_test!(rv64ui_p_or);
isa_test!(rv64ui_p_ori);
isa_test!(rv64ui_p_sb, { 16 * KB });
isa_test!(rv64ui_p_sd, { 16 * KB });
isa_test!(rv64ui_p_sh, { 16 * KB });
isa_test!(rv64ui_p_sll);
isa_test!(rv64ui_p_slli);
isa_test!(rv64ui_p_slliw);
isa_test!(rv64ui_p_sllw);
isa_test!(rv64ui_p_slt);
isa_test!(rv64ui_p_slti);
isa_test!(rv64ui_p_sltu);
isa_test!(rv64ui_p_sra);
isa_test!(rv64ui_p_srai);
isa_test!(rv64ui_p_sraiw);
isa_test!(rv64ui_p_sraw);
isa_test!(rv64ui_p_srl);
isa_test!(rv64ui_p_srli);
isa_test!(rv64ui_p_srliw);
isa_test!(rv64ui_p_srlw);
isa_test!(rv64ui_p_sub);
isa_test!(rv64ui_p_subw);
isa_test!(rv64ui_p_sw, { 16 * KB });
isa_test!(rv64ui_p_xor);
isa_test!(rv64ui_p_xori);
isa_test!(rv64um_p_div);
isa_test!(rv64um_p_divu);
isa_test!(rv64um_p_divuw);
isa_test!(rv64um_p_divw);
isa_test!(rv64um_p_mul);
isa_test!(rv64um_p_mulh);
isa_test!(rv64um_p_mulhsu);
isa_test!(rv64um_p_mulhu);
isa_test!(rv64um_p_mulw);
isa_test!(rv64um_p_rem);
isa_test!(rv64um_p_remu);
isa_test!(rv64um_p_remuw);
isa_test!(rv64um_p_remw);