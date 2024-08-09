use std::{fs, process::exit};

use elf_load::Elf;

use crate::{
    hart::privilege::PrivilegeMode,
    memory::pmp::PMP,
    memory::{KB, MB},
    vmstate::VMState,
    vmstate::VMStateBuilder,
};

isa_test!(off(tohost: 0x80001000u64): rv64ui_p_add);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_addi);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_addiw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_addw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_and);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_andi);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_auipc);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_beq);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_bge);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_bgeu);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_blt);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_bltu);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_bne);
// isa_test!(off(tohost: 0x80001000u64): rv64ui_p_fence_i, "rv64ui-p-fence_i", { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_jal);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_jalr);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_lb, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_lbu, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_ld, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_lh, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_lhu, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_lui);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_lwu, { 16 * KB });
// isa_test!(off(tohost: 0x80001000u64): rv64ui_p_ma_data, "rv64ui-p-ma_data", { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_or);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_ori);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sb, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sd, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sh, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sll);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_slli);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_slliw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sllw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_slt);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_slti);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sltu);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sra);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_srai);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sraiw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sraw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_srl);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_srli);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_srliw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_srlw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sub);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_subw);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_sw, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_xor);
isa_test!(off(tohost: 0x80001000u64): rv64ui_p_xori);

isa_test!(off(tohost: 0x80001000u64): rv64um_p_div);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_divu);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_divuw);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_divw);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_mul);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_mulh);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_mulhsu);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_mulhu);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_mulw);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_rem);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_remu);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_remuw);
isa_test!(off(tohost: 0x80001000u64): rv64um_p_remw);

isa_test!(off(tohost: 0x80001000u64): rv64ua_p_lrsc, {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoswap_w, "rv64ua-p-amoswap_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoswap_d, "rv64ua-p-amoswap_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoadd_w, "rv64ua-p-amoadd_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoadd_d, "rv64ua-p-amoadd_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoand_w, "rv64ua-p-amoand_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoand_d, "rv64ua-p-amoand_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoor_w, "rv64ua-p-amoor_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoor_d, "rv64ua-p-amoor_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoxor_w, "rv64ua-p-amoxor_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amoxor_d, "rv64ua-p-amoxor_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amomax_w, "rv64ua-p-amomax_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amomax_d, "rv64ua-p-amomax_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amomaxu_w, "rv64ua-p-amomaxu_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amomaxu_d, "rv64ua-p-amomaxu_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amomin_w, "rv64ua-p-amomin_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amomin_d, "rv64ua-p-amomin_d", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amominu_w, "rv64ua-p-amominu_w", {16 * KB});
isa_test!(off(tohost: 0x80001000u64): rv64ua_p_amominu_d, "rv64ua-p-amominu_d", {16 * KB});

isa_test!(off(tohost: 0x80001000u64): rv64si_p_csr, { 16 * KB });
isa_test!(off(tohost: 0x80001000u64): rv64si_p_scall, { 16 * KB });

#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fadd, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fclass);
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fcmp, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fcvt, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fcvt_w, "rv64uf-p-fcvt_w", {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fdiv, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fmadd, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_fmin, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_ldst, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_move, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64uf_p_recoding, {16 * KB});

#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fadd, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fclass);
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fcmp, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fcvt, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fcvt_w, "rv64uf-p-fcvt_w", {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fdiv, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fmadd, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_fmin, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_ldst, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80002000u64): rv64ud_p_move, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_recoding, {16 * KB});
#[cfg(feature = "float")]
isa_test!(off(tohost: 0x80001000u64): rv64ud_p_structural, {16 * KB});

isa_test!(off(tohost: 0x80003000u64): rv64uc_p_rvc, {16 * KB});
