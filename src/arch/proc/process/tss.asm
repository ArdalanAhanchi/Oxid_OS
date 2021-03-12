; Some basic functions to allow the manipulation of the TSS and their
; corresponding entries in the GDT. This is done since this OS does not
; have comprehensive GDT management code (not really needed in long mode).
;
; Author: Ardalan Ahanchi
; Date: March 2021

; The calling of these functions and the calling conventions are System V AMD64.

global load_task_register
global get_gdt_addr
global get_gdt_tss_offset

; To be able to access the gdt and tss offset.
extern gdt
extern gdt.tss

; A sub-routine which calls the ltr instruction with the offset which is stored
; in the ax (16 bits only).
load_task_register:
    mov rax, rdi
    ltr ax
    ret

; A sub-routine which returns the current address of the GDT.
get_gdt_addr:
    mov rax , gdt
    ret

; A sub-routine which returns the offset of the 1st tss entry in the GDT.
get_gdt_tss_offset:
    mov rax, gdt.tss
    ret
