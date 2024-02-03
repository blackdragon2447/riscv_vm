#include "../common.S"

reset_vector:
	la a0, mtvec
	csrw mtvec, a0
	csrr t0, mhartid
	bnez t0, park

	j pass

