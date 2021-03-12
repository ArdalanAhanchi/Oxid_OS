; Some wrappers for assembly instructions to allow accessing the IO ports to
; provide communication with the PIC chips.
;
; Author: Ardalan Ahanchi
; Date: Feb 2021

; The calling of these functions and the calling conventions are System V AMD64.

global out_b
global in_b

; A wrapper for the out instruction which writes a byte. The first argument is
; a 16-bit value representing the io port, the second is a byte long value.
out_b:
    mov rdx, rdi        ; Store the port number in rdx.
    mov rax, rsi        ; Store the value in rax.
    out dx, al          ; Call and return.
    ret

; A wrapper for the in instruction which reads a byte from an IO port. It
; accepts a 16-bit io port as a parameter, and returns an 8 bit value.
in_b:
    mov rdx, rdi        ; Store the port number in rdi.
    in al, dx           ; Call in and store the value in rax.
    ret                 ; Since the return value is in al anyways, we can ret.
