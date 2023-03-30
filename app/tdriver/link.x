MEMORY
{
  FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 64K
  RAM (rwx) : ORIGIN = 0x000010000, LENGTH = 32K
  /* Memory mapped io */
  SCREEN (w) : ORIGIN = 0x1E000, LENGTH = 8K
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
