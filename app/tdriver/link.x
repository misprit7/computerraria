MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 768K
  RAM (rwx) : ORIGIN = 0x00100000, LENGTH = 368K
  /* Memory mapped io */
  SCREEN (w) : ORIGIN = 0x00200000, LENGTH = 16K
  BW_SCREEN (rw) : ORIGIN = 0x0020E000, LENGTH = 8K
}

/* The entry point is the reset handler */
ENTRY(Reset);

SECTIONS
{
  .text :
  {
    KEEP(*(.text.start));
    *(.text.reset);
    *(.text .text.*);
  } > FLASH

  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = .;
  } > RAM

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))
  {
    _sdata = .;
    *(.data .data.*);
    _edata = .;
  } > RAM

  _sidata = LOADADDR(.data);
  _stack_start = ORIGIN(RAM) + LENGTH(RAM);


}
