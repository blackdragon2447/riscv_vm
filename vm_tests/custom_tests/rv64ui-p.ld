OUTPUT_ARCH( "riscv" )

ENTRY( _start )

MEMORY
{
  /* test_out (wa!ri) : ORIGIN = 0x70000000, LENGTH = 128 */
  /* ram   (wxa!ri) : ORIGIN = 0x80000000, LENGTH = 128K */
  test_out (wa) : ORIGIN = 0x70000000, LENGTH = 128
  ram   (wxa) : ORIGIN = 0x80000000, LENGTH = 128K
}

PHDRS
{
  text PT_LOAD;
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

