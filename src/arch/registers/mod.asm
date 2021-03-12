; Some wrappers for assembly instructions to allow switching page tables.
;
; Author: Ardalan Ahanchi
; Date: Feb 2021

; The calling of these functions and the calling conventions are System V AMD64.

section .text

; Some macros to ease the implementation of getters and setters ;;;;;;;;;;;;;;;;

; A macro which implements a setter for a given register (as first parameter).
%macro impl_setter 1
    global set_%1                   ; Make it accessible to rust.

    ; A routine which sets the %1 register to whatever is in the first argument.
    set_%1:                         ; Create a label.
        mov %1, rdi                 ; Set and return.
        ret
%endmacro

; A macro which implements a getter for a given register (returned in rax).
%macro impl_getter 1
    global get_%1                   ; Make it accessible to rust.

    ; A routine which gets the %1 register and returns it.
    get_%1:                         ; Create a label.
        mov rax, %1                 ; Get, store, and return.
        ret
%endmacro

; A macro which implements both a getter and setter for a given register.
%macro impl_accessors 1
    impl_getter %1
    impl_setter %1
%endmacro

; Actual accessors are implemented here ;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;

; Implement accessors for general purpose registers.
impl_accessors rax
impl_accessors rbx
impl_accessors rcx
impl_accessors rdx
impl_accessors rbp
impl_accessors rsp
impl_accessors rsi
impl_accessors rdi
impl_accessors r8
impl_accessors r9
impl_accessors r10
impl_accessors r11
impl_accessors r12
impl_accessors r13
impl_accessors r14
impl_accessors r15

; Implement accessors for control registers.
impl_accessors cr2
impl_accessors cr3

; Implement getters for the segment registers.
impl_getter cs
