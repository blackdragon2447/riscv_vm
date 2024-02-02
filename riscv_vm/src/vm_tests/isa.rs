use std::{fs, process::exit};

use elf_load::Elf;

use crate::{
    hart::privilege::PrivilegeMode,
    memory::pmp::PMP,
    memory::{KB, MB},
    vmstate::VMState,
    vmstate::VMStateBuilder,
};

isa_test!(off: rv64ui_p_add);
isa_test!(off: rv64ui_p_addi);
isa_test!(off: rv64ui_p_addiw);
isa_test!(off: rv64ui_p_addw);
isa_test!(off: rv64ui_p_and);
isa_test!(off: rv64ui_p_andi);
isa_test!(off: rv64ui_p_auipc);
isa_test!(off: rv64ui_p_beq);
isa_test!(off: rv64ui_p_bge);
isa_test!(off: rv64ui_p_bgeu);
isa_test!(off: rv64ui_p_blt);
isa_test!(off: rv64ui_p_bltu);
isa_test!(off: rv64ui_p_bne);
// isa_test!(off: rv64ui_p_fence_i, "rv64ui-p-fence_i", { 16 * KB });
isa_test!(off: rv64ui_p_jal);
isa_test!(off: rv64ui_p_jalr);
isa_test!(off: rv64ui_p_lb, { 16 * KB });
isa_test!(off: rv64ui_p_lbu, { 16 * KB });
isa_test!(off: rv64ui_p_ld, { 16 * KB });
isa_test!(off: rv64ui_p_lh, { 16 * KB });
isa_test!(off: rv64ui_p_lhu, { 16 * KB });
isa_test!(off: rv64ui_p_lui);
isa_test!(off: rv64ui_p_lwu, { 16 * KB });
// isa_test!(off: rv64ui_p_ma_data, "rv64ui-p-ma_data", { 16 * KB });
isa_test!(off: rv64ui_p_or);
isa_test!(off: rv64ui_p_ori);
isa_test!(off: rv64ui_p_sb, { 16 * KB });
isa_test!(off: rv64ui_p_sd, { 16 * KB });
isa_test!(off: rv64ui_p_sh, { 16 * KB });
isa_test!(off: rv64ui_p_sll);
isa_test!(off: rv64ui_p_slli);
isa_test!(off: rv64ui_p_slliw);
isa_test!(off: rv64ui_p_sllw);
isa_test!(off: rv64ui_p_slt);
isa_test!(off: rv64ui_p_slti);
isa_test!(off: rv64ui_p_sltu);
isa_test!(off: rv64ui_p_sra);
isa_test!(off: rv64ui_p_srai);
isa_test!(off: rv64ui_p_sraiw);
isa_test!(off: rv64ui_p_sraw);
isa_test!(off: rv64ui_p_srl);
isa_test!(off: rv64ui_p_srli);
isa_test!(off: rv64ui_p_srliw);
isa_test!(off: rv64ui_p_srlw);
isa_test!(off: rv64ui_p_sub);
isa_test!(off: rv64ui_p_subw);
isa_test!(off: rv64ui_p_sw, { 16 * KB });
isa_test!(off: rv64ui_p_xor);
isa_test!(off: rv64ui_p_xori);
isa_test!(off: rv64um_p_div);
isa_test!(off: rv64um_p_divu);
isa_test!(off: rv64um_p_divuw);
isa_test!(off: rv64um_p_divw);
isa_test!(off: rv64um_p_mul);
isa_test!(off: rv64um_p_mulh);
isa_test!(off: rv64um_p_mulhsu);
isa_test!(off: rv64um_p_mulhu);
isa_test!(off: rv64um_p_mulw);
isa_test!(off: rv64um_p_rem);
isa_test!(off: rv64um_p_remu);
isa_test!(off: rv64um_p_remuw);
isa_test!(off: rv64um_p_remw);

isa_test!(off: rv64si_p_csr, { 16 * KB });
isa_test!(off: rv64si_p_scall, { 16 * KB });