//! A sub-module which represents an IDT entry. It also provides helpful functionality to 
//! set various attributes in the entry, and translate addresses to it's format.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

use crate::mem::bitwise::BitWise;       // To allow easy bitwise operations.

/// Corresponds to the first nibble of IDTEnt::attr_type, it represents an interrupt gate. Which
/// means that further interrupts are masked, and the execution will resume to the next instruction.
const TYPE_INTERRUPT_GATE: u8 = 0xE;

/// Corresponds to the first nibble of IDTEnt::attr_type, it represents a trap gate. Which means 
/// that further interrupts are enabled, and the execution will resume to the current instruction.
const TYPE_TRAP_GATE: u8 = 0xF;

/// A structure which represents a single IDT entry. It is mainly for representing the address of 
/// the interrupt handler (ex. as interrupt service routine).
/// More details can be found at: https://wiki.osdev.org/IDT#Structure_AMD64
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IDTEnt {
    offset_0_15: u16,           // First chunk of the ptr.
    selector: u16,              // Code segment selector (will always be cs in x86 long mode).
    ist: u8,                    // Interrupt stack table offset (only bits 0 to 2).
    attr_type: u8,              // Type of interrupt and it's attributes.
    offset_16_31: u16,          // Second part of the ptr.
    offset_32_63: u32,          // Third chunk of the ptr.
    zero: u32,                  // Not used, included for specifications.
}

impl IDTEnt {
    /// A constant constructor which simply initializes everything and sets all to 0. This is used
    /// for population of IDT before the runtime (since it's static).
    ///
    /// # Returns
    /// An empty IDT Entry where it is also not present (since every bit is set to 0).
    pub const fn new_empty() -> Self {
        // Create an entry with everything set to zeros (automatically non-present).
        IDTEnt {
            offset_0_15: 0,
            selector: 0,
            ist: 0,
            attr_type: TYPE_INTERRUPT_GATE,
            offset_16_31: 0,
            offset_32_63: 0,
            zero: 0,
        }
    }
    
    /// A method which stores the address of function pointed to by ptr to the offset bits
    /// in this interrupt decriptor entry. It divides it into the three parts as specified 
    /// by the x86_64 specifications.
    ///
    /// # Parameters
    /// `handler` : The pointer for the function which should handle the interrupt.
    pub unsafe fn set_handler(&mut self, handler: unsafe extern "sysv64" fn()) {
        // Cast, and shift the bits if necessary to use all the 64 bits of data.
        self.offset_0_15 = (handler as usize) as u16;
        self.offset_16_31 = ((handler as usize) >> 16) as u16;
        self.offset_32_63 = ((handler as usize) >> 32) as u32;
    }
    
    /// A method which reads the current handler, and converts it to a function pointer and returns 
    /// it. It is used to confirm that the set_handler works as expected.
    ///
    /// # Returns
    /// A function pointer to the interrupt service routine which was set by set_handler.
    pub unsafe fn get_handler(&mut self) -> unsafe extern "sysv64" fn() {
        // Cast, and shift the bits if necessary to use all the 64 bits of data. 
        let mut fn_ptr: usize = self.offset_0_15 as usize;
        fn_ptr |= (self.offset_16_31 as usize) << 16;
        fn_ptr |= (self.offset_32_63 as usize) << 32;
    
        // Cast the pointer to a function, and return it.
        core::mem::transmute(fn_ptr)
    }
    
    /// A method which loads the segment selector from the cs register into the selector.
    /// internally, it calls an assembly function to get the value of cs.
    pub unsafe fn load_selector(&mut self) { self.selector = crate::arch::registers::get_cs(); }
    
    /// A method which sets the present bit of this idt entry.
    pub unsafe fn set_present(&mut self) { self.attr_type.set_bit(7); }
    
    /// A method which clears the present bit of this idt entry.
    pub unsafe fn set_absent(&mut self) { self.attr_type.clear_bit(7); }
    
    /// A function which sets the type of this entry as an interrupt. Which means that when the 
    /// isr is called, the interrupts will be disabled by the system.
    pub unsafe fn set_interrupt_type(&mut self) { 
        // First clear the first nibble.
        self.attr_type &= 0xF0;
        
        // Then set it to the interrupt type.
        self.attr_type |= TYPE_INTERRUPT_GATE;
    }
    
    /// A function which sets the type of this entry as a gate, Which means that when the 
    /// isr is called, the interrupts will be enabled by the system.
    pub unsafe fn set_trap_type(&mut self) { 
        // First clear the first nibble.
        self.attr_type &= 0xF0;
        
        // Then set it to the trap type.
        self.attr_type |= TYPE_TRAP_GATE;
    }
    
    /// A method which sets the descriptor privilage level (minimum level of calling descriptor).
    /// the dpl can be 0 to 3 (all the available rings). So only the first two bits of it will be
    /// utilized and the rest will be masked.
    ///
    /// # Parameters
    /// `dpl` : The descriptor privilage level. A value between 0 to 3 (inclusive).
    pub unsafe fn set_dpl(&mut self, dpl: u8) {
        // Define the bitmask for the dpl.
        const DPL_BITMASK: u8 = 0b01100000;
    
        // Shift the passed dpl, and mask it to remove unnecessary bits.
        let masked_dpl: u8 = (dpl << 5) & (DPL_BITMASK);
        
        // Clear the currently stored type attr.
        self.attr_type &= !DPL_BITMASK;
        
        // Set the masked dpl to the correct bits.
        self.attr_type |= masked_dpl;
    }
    
    /// A simple setter for the interrupt stack table. If the value stored is 0, it will not 
    /// change the stack. Otherwise, it's an offset into the IST. Additionally, only 3 bits of 
    /// it are used. Values larger will be masked.
    ///
    /// # Parameters
    /// `offset` : 0 for the same stack, other values up to 7 as the stack number.
    pub unsafe fn set_ist(&mut self, offset: u8) {
        // Define the bitmask for the ist.
        const IST_BITMASK: u8 = 0b00000111;
        
        // First clear the current value stored in IST.
        self.ist &= !IST_BITMASK;
        
        // Then set the bitmasked value of offset.
        self.ist |= offset & IST_BITMASK;
    }
}
