# no common since we want a custom mtvec
# #include "../common.S"

.global _start

.equ	INT_CONTROL, 0x2000

_start:
	j reset_vector

mtvec:
	csrr t5, mcause
	bgez t5, mtvec.exept
	j mtvec.int
mtvec.exept:
	li t6,8
	beq t5, t6, write_tohost
	li t6,9
	beq t5, t6, write_tohost
	li t6,11
	beq t5, t6, write_tohost
	slli t5, t5, 12
	mv gp, t5
	slli gp, gp, 0x1
	ori gp, gp, 0x1
	j write_tohost
mtvec.int:
	slli t5, t5, 1
	srli t5, t5, 1
	li t6, 3
	beq t5, t6, pass
	j fail

write_tohost:
	la t5, _test_out
	sw gp,0(t5)
	; auipc t5, 0x1
	; sw zero,-8(t5)
	j write_tohost

park:
	wfi
	j park

pass: 
	fence
	li gp,1
	ecall
	unimp

fail: 
	fence
	slli gp, gp, 0x1
	ori gp, gp, 0x1
	ecall
	unimp

reset_vector:
	la a0, mtvec
	csrw mtvec, a0

	li t0, 0x8
	csrs mstatus, t0
	li t0, 0xA
	csrs mie, t0

	csrr t0, mhartid
	bnez t0, park

	li t1, INT_CONTROL	
	addi t1, t1, (8*1)
	li t2, 1
	sw t2,0(t1)
	j park
