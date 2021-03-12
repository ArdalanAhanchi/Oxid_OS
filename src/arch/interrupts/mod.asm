; Some wrappers for assembly instructions to allow the implementation of
; interrupts.
;
; Author: Ardalan Ahanchi
; Date: Feb 2021

; The calling of these functions and the calling conventions are System V AMD64.

global enable
global disable

; A simple wrapper for the STI instruction.
enable:
    sti				; Simply call it and return.
    ret

; A simple wrapper for the STI instruction.
disable:
    cli				; Simply call it and return.
    ret
