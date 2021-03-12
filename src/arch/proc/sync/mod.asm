; Some wrappers for assembly instructions to allow the implementation of
; spin-lock based mutexes and other syncrhonization primitives.
;
; Author: Ardalan Ahanchi
; Date: Feb 2021

; The calling of these functions and the calling conventions are System V AMD64.
global spin_lock
global atomic_bool_lock_cmpxchg

; An implementation of a fairly performant spin lock mechanism. It's definition
; is based on the sysv64 ABI. It accepts a pointer to an unsigned integer
; value. And locks until the value becomes 0. RDI is ptr.
spin_lock:
    mov cl, 1                  ; The "new" parameter in cmpxchg (True).
.retry:
    mov al, 0                  ; The "expected" parameter in cmpxchg (False).
    lock cmpxchg [rdi], cl     ; "ptr" is the first argument (in rdi).
    jnz spin_lock.waiting      ; Go to the waiting if not finished (ZF is 0).
    jmp spin_lock.finished     ; Go to the finished state (we're done).
.waiting:
    pause                      ; Pause and continue the loop.
    jmp spin_lock.retry
.finished:                     ; We're done and we can return.
    ret

; A wrapper for the cmpxchg instruction which is used for the implementation of
; the atmoic compare_and_swap function.
; https://en.wikipedia.org/wiki/Compare-and-swap
; http://heather.cs.ucdavis.edu/~matloff/50/PLN/lock.pdf
; https://www.assemblylanguagetuts.com/x86-assembly-registers-explained/
atomic_bool_lock_cmpxchg:
    ; RDI is ptr, RSI is expected, RDX is new
    ; Put the expected value in the accumulator (rax)
    mov rax, rsi
    ; Run the instruction with dst (ptr), and src (new).
    lock cmpxchg [rdi], dl
    ; Reset the rax register fully.
    mov rax, QWORD 0
    ; If the ZF flag is 0, we can return 0 (which is "false" and already in rax).
    jnz atomic_bool_lock_cmpxchg.finished
    ; If the ZF flag was not 0, we have to return true. So set it to 1.
    mov rax, QWORD 1
.finished:
    ret
