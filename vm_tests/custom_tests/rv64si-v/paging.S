#include "../common.S"

# ppns
.equ	PAGE_TABLE_L0,	0x80002
.equ	PAGE_TABLE_L1,	0x80003
.equ	PAGE_TABLE_L2,	0x80004
.equ	U_MEM_START,	0x80200

.equ	PTE_VALID,	0b00000001
.equ	PTE_READ,	0b00000010
.equ	PTE_WRITE,	0b00000100
.equ	PTE_EXEC,	0b00001000
.equ	PTE_USER,	0b00010000
.equ	PTE_GLOBAL,	0b00100000
.equ	PTE_ACCESS,	0b01000000
.equ	PTE_DIRTY,	0b10000000

stvec:
	csrr a2, scause
	li t5, 5
	beq a2, t5, stvec.fail
	li t5, 7
	beq a2, t5, stvec.fail
	li t5, 13
	beq a2, t5, stvec.fail
	li t5, 15
	beq a2, t5, stvec.fail
	li t5, 8
	beq a2, t5, stvec.check_umem
stvec.check_umem:
	li t0, 1	
	slli t0, t0, 18
	csrs sstatus, t0
	li a4, 1
	slli a4, a4, 21

	li t1, 0xdead
	ld a5, 0(a4)
	beq a5, t1, stvec.pass
stvec.fail:
	j fail
stvec.pass:
	la a0, s_mode_cont
	la a1, _memory_start
	sub a0, a0, a1
	csrw sepc, a0

	li a0, 1
	slli a0, a0, 8
	csrs sstatus, a0
	sret

reset_vector:
	la a0, mtvec
	csrw mtvec, a0
	csrr t0, mhartid
	bnez t0, park

	# Create a pte (0) and set it's 
	# valid bit.
	li a0, %lo(PTE_VALID)

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	lui	a1, %hi(PAGE_TABLE_L1)
	addi a1, a1, %lo(PAGE_TABLE_L1)

	# Load the ppn of the root page table 
	# and converto it into an addr.
	lui	a2, %hi(PAGE_TABLE_L0)
	addi a2, a2, %lo(PAGE_TABLE_L0)
	slli a2, a2, 12

	# load the index of this PTE
	li a3, 0

	jal ra, put_pte

	# Create a pte (0) and set it's 
	# valid bit.
	li a0, %lo(PTE_VALID)

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	lui	a1, %hi(PAGE_TABLE_L2)
	addi a1, a1, %lo(PAGE_TABLE_L2)

	# Load the ppn of the root page table 
	# and converto it into an addr.
	lui	a2, %hi(PAGE_TABLE_L1)
	addi a2, a2, %lo(PAGE_TABLE_L1)
	slli a2, a2, 12

	# load the index of this PTE
	li a3, 0

	jal ra, put_pte

	# Create a pte (0) and set it's 
	# valid bit.
	li a0, %lo(PTE_VALID)
	ori a0, a0, %lo(PTE_EXEC)
	ori a0, a0, %lo(PTE_ACCESS)

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	la a1, _memory_start
	srli a1, a1, 12

	# Load the ppn of the root page table 
	# and converto it into an addr.
	lui	a2, %hi(PAGE_TABLE_L2)
	addi a2, a2, %lo(PAGE_TABLE_L2)
	slli a2, a2, 12

	# load the index of this PTE
	li a3, 0

	jal ra, put_pte

	# Create a pte (0) and set it's 
	# valid bit.
	li a0, %lo(PTE_VALID)
	ori a0, a0, %lo(PTE_EXEC)
	ori a0, a0, %lo(PTE_USER)
	ori a0, a0, %lo(PTE_ACCESS)

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	la a1, _user_start
	srli a1, a1, 12

	# Load the ppn of the root page table 
	# and converto it into an addr.
	lui	a2, %hi(PAGE_TABLE_L2)
	addi a2, a2, %lo(PAGE_TABLE_L2)
	slli a2, a2, 12

	# load the index of this PTE
	li a3, 1

	jal ra, put_pte

	#################################
	# Setup pages for the page table
	# These should be read/write by S Mode

	li a0, %lo(PTE_VALID)
	ori a0, a0, %lo(PTE_READ)
	ori a0, a0, %lo(PTE_WRITE)
	ori a0, a0, %lo(PTE_ACCESS)
	ori a0, a0, %lo(PTE_DIRTY)

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	lui	a1, %hi(PAGE_TABLE_L0)
	addi a1, a1, %lo(PAGE_TABLE_L0)

	# Load the ppn of the root page table 
	# and converto it into an addr.
	lui	a2, %hi(PAGE_TABLE_L2)
	addi a2, a2, %lo(PAGE_TABLE_L2)
	slli a2, a2, 12

	# load the index of this PTE
	li a3, 2

	jal ra, put_pte

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	lui	a1, %hi(PAGE_TABLE_L1)
	addi a1, a1, %lo(PAGE_TABLE_L1)

	# load the index of this PTE
	li a3, 3

	jal ra, put_pte

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	lui	a1, %hi(PAGE_TABLE_L2)
	addi a1, a1, %lo(PAGE_TABLE_L2)

	# load the index of this PTE
	li a3, 4

	jal ra, put_pte

	##############################
	# Prepare for the switch to S Mode

	li a0, 256
	ori a0, a0, 32
	ori a0, a0, 128
	li t1, 1
	slli t1, t1, 13
	or a0, a0, t1
	li t1, 1
	slli t1, t1, 15
	or a0, a0, t1
	csrs medeleg, a0

	la t1, _memory_start
	la a0, stvec
	sub a0, a0, t1
	csrw stvec, a0

	lui	a0, %hi(PAGE_TABLE_L0)
	addi a0, a0, %lo(PAGE_TABLE_L0)

	li a1, 8
	slli a1, a1, 60
	or a0, a0, a1
	csrw satp, a0

	li a0, 0b01
	slli a0, a0, 11
	csrs mstatus, a0

	la a0, s_mode
	la a1, _memory_start
	sub a0, a0, a1
	csrw mepc, a0
	mret

s_mode:

	# Create a pte (0) and set it's 
	# valid bit.
	li a0, %lo(PTE_VALID)
	ori a0, a0, %lo(PTE_READ)
	ori a0, a0, %lo(PTE_WRITE)
	ori a0, a0, %lo(PTE_USER)
	ori a0, a0, %lo(PTE_ACCESS)
	ori a0, a0, %lo(PTE_DIRTY)

	# Load the ppn for the page table the 
	# first entry of the root points to 
	# and put it in the PTE
	lui	a1, %hi(U_MEM_START)
	addi a1, a1, %lo(U_MEM_START)

	# Load the ppn of the root page table 
	# and converto it into an addr.
	li a2, 3
	slli a2, a2, 12

	# load the index of this PTE
	li a3, 1

	jal ra, put_pte

	li a0, 0b0
	slli a0, a0, 8
	csrs sstatus, a0

	la a0, u_mode
	la a1, _memory_start
	sub a0, a0, a1
	csrw sepc, a0
	sret

s_mode_cont:
	li a0, 1
	slli a0, a0, 18
	csrc sstatus, a0
	slli a0, a0, 1
	csrs sstatus, a0

	li a0, 0x2fc
	lw a1, 0(a0)
	li a2, 0x10200073
	bne a1, a2, s_mode_cont.fail
s_mode_cont.pass:
	j pass

s_mode_cont.fail:
	j fail
	
# a0 contains the bits that need to be set
# a1 contains the place it points to
# a2 contains the address of the pte 
#	it needs to be put into
# a3 contains its index
put_pte:
	slli a1, a1, 10
	or t2, a0, a1

	li t0, 8
	mul a3, a3, t0
	add t1, a2, a3
	sd t2, 0(t1)

	ret

.section .user, "ax"

u_mode:

	li a0, 1
	slli a0, a0, 21

	li a1, 0xdead

	sd a1, 0(a0)

	ecall

