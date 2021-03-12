; Some wrappers for assembly instructions to allow the implementation of
; idt (specifically loading it).
;
; Author: Ardalan Ahanchi
; Date: Feb 2021

; The calling of these functions and the calling conventions are System V AMD64.

global load_idt
global store_idt

; A wrapper for the LIDT instruction. It loads the IDT into memory. The address
; passed will be stored in the rdi as specified by sysv ABI.
load_idt:
    lidt [rdi]        ; Call the lidt with the passed address and return.
    ret

; A wrapper for the SIDT instruction. It stores the IDT into the IDTPtr
; structure pointed to by the first argument (rdi based on sysv ABI).
store_idt:
    sidt [rdi]        ; Call the lidt with the passed address and return.
    ret
