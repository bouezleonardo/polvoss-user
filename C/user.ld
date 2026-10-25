/* Specify the architecture*/
OUTPUT_ARCH("riscv")

/* Symbol of the entry point of the program */
ENTRY(_start)

/* Specify the sections of the final executable
where the sections of the object files will be put */
SECTIONS
{
  /* Start virtual address offset */
  . = 0x0;
  
  /* Program code */
  .text : 
  {
    *(.text .text.*)
  }
  
  /* Read-only data i.e. constants*/
  .rodata : 
  {
    . = ALIGN(16);
    *(.srodata .srodata.*)
    . = ALIGN(16);
    *(.rodata .rodata.*)
  }

  /* Read-write initialized data */
  . = ALIGN(0x1000);
  .data : 
  {
    . = ALIGN(16);
    *(.sdata .sdata.*)
    . = ALIGN(16);
    *(.data .data.*)
  }
  
  /* Read-write uninitialized or 
  zero initialized data */
  .bss : 
  {
    . = ALIGN(16);
    *(.sbss .sbss.*)
    . = ALIGN(16);
    *(.bss .bss.*)
  }

  PROVIDE(end = .);
}
