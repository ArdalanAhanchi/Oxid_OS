//! A sub-module which handles the Interrupt descriptor table as defined in the x86-64
//! architecture. It handles the 
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

mod idt_ent;            // For representing the entries.
use idt_ent::IDTEnt;    // Bring it into scope

mod isr;                // For linking the isr's to the main handler.

/// A constant which corresponds to the total number of IDT entries (not only the ones used).
pub const NUM_IDT_ENTRIES: usize = 256;

/// Define an empty interrupt descriptor table with everything set to 0.
pub static mut IDT: [IDTEnt; NUM_IDT_ENTRIES] = [IDTEnt::new_empty(); NUM_IDT_ENTRIES];


extern "sysv64" {
    /// A function which loads the IDT. It is a wrapper for the LIDT instruction.
    /// it utilizes the System V AMD64 calling conventions to call the assembly code.
    ///
    /// # Parameters
    /// `idt_addr` : The linear address of the interrupt descriptor table.
    fn load_idt(idt_ptr: *const IDTPtr);
    
    /// A wrapper for the SIDT instruction. It stores the IDT into the IDTPtr
    /// structure pointed to by the first argument (rdi based on sysv ABI).
    ///
    /// # Parameters
    /// `idt_ptr` : The linear address of the interrupt descriptor table pointer structure.
    fn store_idt(idt_ptr: *const IDTPtr);
}

/// A structure which represents the IDT ptr. It is used for loading the IDT to the system.
/// It's definition is specified by the x86_64 architecutre.
/// More details can be found at: https://wiki.osdev.org/IDT#Structure_AMD64
#[repr(C, packed)]
struct IDTPtr {
    pub size: u16,              // The total size of the IDT (in bytes) - 1.
    pub addr: usize,            // The starting address of IDT.
}

/// A function which loads the interrupt descriptor table to the system. It creates a new IDTPtr
/// struct with the correct size and address, a
pub unsafe fn init() {
    // Initialize the used entries with their selectors and present bit.
    for i in 0..NUM_IDT_ENTRIES {
        IDT[i].load_selector();
        IDT[i].set_present();
    }
    
    // Register all the assembly wrappers with their corresponding rust handler.
    isr::register();
    
    // Create a new idt ptr structure. Calculate the size, and get the IDT ptr and cast it.
    let idt_ptr = IDTPtr {
        size: ((NUM_IDT_ENTRIES * core::mem::size_of::<IDTEnt>()) - 1) as u16,
        addr: IDT.as_ptr() as usize,
    };
    
    // Call the assembly load function.
    load_idt(&idt_ptr);
}
