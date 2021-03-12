//! A sub-module which includes all the interrupt handling code. It includes the management of the
//! interrupt descriptor table, the handling of interrupts, and more.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

pub mod idt;
pub mod handlers;
pub mod pic;

pub const IRQ_OFFSET: u8 = 32;       // The offset for hardware interrupts (set by PIC or APIC).

/// A group of signature definitions to allow calling assembly code from rust.
/// it utilizes the System V AMD64 calling conventions to call the assembly code.
extern "sysv64" {
    /// A function which enables interrupts by using the STI instruction.
    pub fn enable();
    
    /// A function which disables interrupts by using the CLI instruction.
    pub fn disable();
}

/// A function which initializes all the interrupt handling code. Including the IDT, and the 
/// programmable interrupt controllers.
pub unsafe fn init() {
    oxid_log!("Initializing interrupts.");

    // Disable interrupts first.
    disable();
    
    // Initiailize the IDT, and setup the first level handlers.
    idt::init();
    
    // Initilize the ISA specified exception handlers (interrupts with number less than 32).
    handlers::exceptions::init();

    // Initialize the PIC.
    pic::init(IRQ_OFFSET);

    // Then enable interrupts.
    enable();
}
