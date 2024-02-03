.global _start

_start:
	j reset_vector

mtvec:
	csrr t5, mcause
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
