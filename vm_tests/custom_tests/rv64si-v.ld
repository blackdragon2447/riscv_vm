OUTPUT_ARCH( "riscv" )

ENTRY( _start )

MEMORY
{
  /* ram   (wxa!ri) : ORIGIN = 0x80000000, LENGTH = 128M */
  test_out (wa) : ORIGIN = 0x70000000, LENGTH = 128
  ram   (wxa) : ORIGIN = 0x80000000, LENGTH = 128M
}

PHDRS
{
  text PT_LOAD;
  user PT_LOAD;
  data PT_LOAD;
  bss PT_LOAD;
}

SECTIONS
{
  .text : {
    PROVIDE(_text_start = .);
    *(.text.init) *(.text .text.*)
    PROVIDE(_text_end = .);
  } >ram AT>ram :text

  .user 0x80001000 : {
    PROVIDE(_user_start = .);
	. = ALIGN(4);
	KEEP(*(.user))
    PROVIDE(_user_end = .);
  } >ram AT>ram :user

  .rodata : {
    PROVIDE(_rodata_start = .);
    *(.rodata .rodata.*)
    PROVIDE(_rodata_end = .);
  } >ram AT>ram :text

  .data : {
    . = ALIGN(4096);
    PROVIDE(_data_start = .);
    *(.sdata .sdata.*) *(.data .data.*)
    PROVIDE(_data_end = .);
  } >ram AT>ram :data

  .bss :{
    PROVIDE(_bss_start = .);
    *(.sbss .sbss.*) *(.bss .bss.*)
    PROVIDE(_bss_end = .);
  } >ram AT>ram :bss

  PROVIDE(_memory_start = ORIGIN(ram));
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));

  PROVIDE(_test_out = ORIGIN(test_out));
}

