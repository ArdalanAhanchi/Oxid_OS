//! A structure which represents the interrupt handler context which is passed from the primary 
//! ISRs (defined in arch/interrupts/idt/isr.asm). A pointer to this structure is passed to the 
//! handler. It provides a context where the high level handler can utilize.
//! 
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

/// A structure which represents the context which is passed to interrupt handlers.
#[repr(C, packed)]
#[derive(Default)]
pub struct Context {
    pub r15: usize,               // All the general purpose registers.
    pub r14: usize,
    pub r13: usize,
    pub r12: usize,
    pub r11: usize,
    pub r10: usize,
    pub r9: usize,
    pub r8: usize,
    pub rdi: usize,
    pub rsi: usize,
    pub rbp: usize,
    pub rdx: usize,
    pub rcx: usize,
    pub rbx: usize,
    pub rax: usize,
    pub err_code: usize,          // 0 if it's not an exception, padded error code otherwise.
    pub rip: usize,               // The last 4 fields are pushed and poped by the CPU.
    pub cs: usize,                // Code segment reigster, padded to become 8 bytes.
    pub rflags: usize,            // The flags register before the interrupt.    
    pub orig_rsp: usize,          // The original stack pointer (if it was switched).
}
