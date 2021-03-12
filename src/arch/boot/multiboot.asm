MULTIBOOT_MAGIC equ 0xe85250d6             ; Magic value specified in multiboot

section .multiboot                         ; The header definition for multiboot

multiboot_begin:
    dd MULTIBOOT_MAGIC                     ; Define the magic value (32-bits)                 
    dd 0                                   ; Architecture number
    dd multiboot_end - multiboot_begin     ; Length of the header
    
    ; Calculate checksum
    dd 0x100000000 - (MULTIBOOT_MAGIC + 0 + (multiboot_end - multiboot_begin))

    ; Multiboot end-tag
    dw 0              
    dw 0
    dd 8
multiboot_end:
