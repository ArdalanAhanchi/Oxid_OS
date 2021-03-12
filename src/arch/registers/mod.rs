//! A sub-module which defines macros to wrap the getter and setters defined in the mod.asm
//! file. This allows rust access to setting or getting the register values.
//! 
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021.

#![allow(unused_macros)]
#![allow(dead_code)]

#[macro_use] 
mod wrapper_macros;                     // A module to allow creating wrappers with one line.


// Define the registers here.

// Wrap the accessors for general purpose registers (64 bits).
wrap_accessors!(get_rax, set_rax, usize);
wrap_accessors!(get_rbx, set_rbx, usize);
wrap_accessors!(get_rcx, set_rcx, usize);
wrap_accessors!(get_rdx, set_rdx, usize);
wrap_accessors!(get_rbp, set_rbp, usize);
wrap_accessors!(get_rsp, set_rsp, usize);
wrap_accessors!(get_rsi, set_rsi, usize);
wrap_accessors!(get_rdi, set_rdi, usize);
wrap_accessors!(get_r8, set_r8, usize);
wrap_accessors!(get_r9, set_r9, usize);
wrap_accessors!(get_r10, set_r10, usize);
wrap_accessors!(get_r11, set_r11, usize);
wrap_accessors!(get_r12, set_r12, usize);
wrap_accessors!(get_r13, set_r13, usize);
wrap_accessors!(get_r14, set_r14, usize);
wrap_accessors!(get_r15, set_r15, usize);

// Wrap the accessors for control registers (64 bits).
wrap_accessors!(get_cr2, set_cr2, usize);
wrap_accessors!(get_cr3, set_cr3, usize);

// Wrap the getter for segment selectors (16 bits long).
wrap_getter!(get_cs, u16);
