ENTRY(_start_32)

SECTIONS {
    . = 1M;

    /* Put the multiboot header section right here at the beginning */
    .boot_header :
    {
        KEEP(build/asm_build/arch/boot/multiboot.o (.multiboot))
    }

    /* All the text (executable instructions) data is next. */
    .text :
    {
        *(.text*)
    }
    
    /* All the read only data is next. */
    .rodata :
    {
        *(.rodata*)
    }
    
    /* All the read and write data is next. */
    .data :
    {
        *(.data*)
    }
    
    /* All the uninitialized data is next. */
    .bss :
    {
    	*(COMMON*)
    	*(.bss*)
    }
}

