; Very basic wrappers to allow halting and pausing functionality to rust.
;
; Author: Ardalan Ahanchi
; Date: Jan 2021

; The calling of these functions and the calling conventions are System V AMD64.
global pause
global halt

; A wrapper for the pause instruction which is used for busy waiting (makes it
; slightly more efficient accross cores).
pause:
    pause                       ; Just simply call the pause instruction.
    ret

; A wrapper for the hlt instruction which simply puts the CPU in low power mode.
halt:
    hlt
    jmp halt
