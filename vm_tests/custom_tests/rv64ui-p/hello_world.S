#include "../common.S"

reset_vector:
	la a0, mtvec
	csrw mtvec, a0
	csrr t0, mhartid
	bnez t0, park

	lui a0, 0x10000
	la a1, hello_world
	lbu t0, 0(a1)
	sd t0, 0(a0)
	addi a1, a1, 1
	bnez t0, reset_vector + 0x0C
	li gp, 1
	j pass

hello_world:
	.string "Hello World!\n"
