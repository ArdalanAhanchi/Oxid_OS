; Some wrappers for assembly instructions to allow tlb management.
;
; Author: Ardalan Ahanchi
; Date: Feb 2021

; The calling of these functions and the calling conventions are System V AMD64.

global flush
global invalidate

; A routine which flushes all the entries in TLB. It simply sets the value in
; CR3 to what it is currently.
flush:
    mov rax, cr3
    mov cr3, rax
    ret

; A routine which flushes a specific address which is passed to this function
; as a first parameter (stored in rdi).
invalidate:
    invlpg [rdi]
    ret
