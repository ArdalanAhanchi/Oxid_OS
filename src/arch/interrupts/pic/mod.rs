//! A sub-module which handles the programmable interrupt controller (8259 PIC). In modern systems
//! this is emulated by the APIC. However, support for it was added due to it's simplicity.
//! More details can be found at: http://www.brokenthorn.com/Resources/OSDevPic.html
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

const PRIMARY_PIC_CMD: u16 = 0x20;           // The command port for the primary PIC.
const PRIMARY_PIC_DATA: u16 = 0x21;          // The data port for the primary PIC.
const SECONDARY_PIC_CMD: u16 = 0xA0;         // The command port for the secondary PIC.
const SECONDARY_PIC_DATA: u16 = 0xA1;        // The data port for the secondary PIC.

const ICW_1: u8 = 0b00010001;                // ICWs used for initializing the PICs.
const ICW_3_PRIMARY: u8 = 0b0100;            // More details about these value can be found at:
const ICW_3_SECONDARY: u8 = 0b0010;          // http://www.brokenthorn.com/Resources/OSDevPic.html
const ICW_4: u8 = 0b0001;

const NUM_IRQS: u8 = 0x8;                    // The number of IRQs for each PIC.
const MAX_IRQS: u8 = 0x10;                   // Total number of IRQs supperted.
const EOI: u8 = 0x20;                        // End of interrupt flag.                  
const DISABLE: u8 = 0xFF;                    // Disable flag.

/// A group of signature definitions to allow calling assembly code from rust.
/// it utilizes the System V AMD64 calling conventions to call the assembly code.
extern "sysv64" {
    /// A function which writes a byte to a given IO port (output port).
    /// 
    /// # Parameters
    /// `io_port` : The port we're writing to (based on x86 specifications).
    /// `value` : The byte we're writing to that port.
    pub fn out_b(io_port: u16, value: u8);
    
    /// A function which reads a byte from a given IO port (input port).
    /// 
    /// # Parameters
    /// `io_port` : The port we're reading from (based on x86 specifications).
    ///
    /// # Returns
    /// The byte which was read from the port.
    pub fn in_b(io_port: u16) -> u8;
}

/// A function which initializes the programmable interrupt controllers and remaps their default
/// mapping. The remapped IRQs start at first_int_num. More details can be found at: 
/// http://www.brokenthorn.com/Resources/OSDevPic.html
///
/// # Parameters
/// `first_int_num` : The interrupt number for irq 0. The maximum int number is first_int_num + 16.
pub unsafe fn init(first_int_num: u8) {
    oxid_log!("Initializing the PIC.");
    
    out_b(PRIMARY_PIC_CMD, ICW_1);                        // Write the ICW_1 to both chips.
    out_b(SECONDARY_PIC_CMD, ICW_1);
    
    out_b(PRIMARY_PIC_DATA, first_int_num);               // Write the starting interrupt number.
    out_b(SECONDARY_PIC_DATA, first_int_num + NUM_IRQS);  // Secondary starts just 8 ints higher.
    
    out_b(PRIMARY_PIC_DATA, ICW_3_PRIMARY);               // Connect the PICs using the IR line.
    out_b(SECONDARY_PIC_DATA, ICW_3_SECONDARY);
    
    out_b(PRIMARY_PIC_DATA, ICW_4);                       // Set the mode for both PICs.
    out_b(SECONDARY_PIC_DATA, ICW_4);
    
    out_b(PRIMARY_PIC_DATA, 0);                           // Clear out the data ports.
    out_b(SECONDARY_PIC_DATA, 0);
    
    // Set the mask for all interrupts by default.
    for irq_num in 0..MAX_IRQS {
         disable_irq(irq_num);
    }
}

/// A function which sends an end of interrupt command to the PIC chipsets based on a given IRQ.
/// It should be called by any function which handles hardware interrupts. More details can be 
/// fonud at : https://wiki.osdev.org/PIC#Programming_the_PIC_chips
///
/// # Parameters
/// `irq_num` : The irq number for this interrupt (typically Interrupt number - IRQ_OFFSET). 
pub unsafe fn end_of_interrupt(irq_num: u8) {
    // Make sure irq_num is in correct range.
    assert!(irq_num < MAX_IRQS, "Invalid IRQ number passed. Out of range.");                

    // Check if it's an interrupt from secondary pic, in that case send EOI to it.
    if irq_num > NUM_IRQS {
        out_b(SECONDARY_PIC_CMD, EOI);
    }
    
    // In both cases, send an EOI to the primary pic.
    out_b(PRIMARY_PIC_CMD, EOI);
}

/// A function which clears the mask for sepcific IRQ number thus it enables it. It follows the 
/// same approach to clear the mask as the C code which can be fonud at
/// https://wiki.osdev.org/PIC#Programming_the_PIC_chips
///
/// # Parameters
/// `irq_num` : The irq number for this interrupt (typically Interrupt number - IRQ_OFFSET). 
pub unsafe fn enable_irq(irq_num: u8) {
    // Make sure irq_num is in correct range.
    assert!(irq_num < MAX_IRQS, "Invalid IRQ number passed. Out of range.");

    // Check which PIC we're working with, and based on that, clear the bit for it.
    if irq_num < NUM_IRQS {
        // Read the currently stored mask, set the value of irq num, and store the new value.
        let new_mask = in_b(PRIMARY_PIC_DATA) & !(1 << irq_num);
        out_b(PRIMARY_PIC_DATA, new_mask);
    } else {
        // If we're here, the secondary PIC is being masked. Make sure to reduce the irq_num.
        let new_mask = in_b(SECONDARY_PIC_DATA) & !(1 << (irq_num - NUM_IRQS));
        out_b(SECONDARY_PIC_DATA, new_mask);
    }
}

/// A function which sets the mask for sepcific IRQ number thus it disables it. It follows the same 
/// approach to set the mask as the C code which can be fonud at
/// https://wiki.osdev.org/PIC#Programming_the_PIC_chips
///
/// # Parameters
/// `irq_num` : The irq number for this interrupt (typically Interrupt number - IRQ_OFFSET). 
pub unsafe fn disable_irq(irq_num: u8) {
    // Make sure irq_num is in correct range.
    assert!(irq_num < MAX_IRQS, "Invalid IRQ number passed. Out of range.");

    // Check which PIC we're working with, and based on that, mask it.
    if irq_num < NUM_IRQS {
        // Read the currently stored mask, set the value of irq num, and store the new value.
        let new_mask = in_b(PRIMARY_PIC_DATA) | (1 << irq_num);
        out_b(PRIMARY_PIC_DATA, new_mask);
    } else {
        // If we're here, the secondary PIC is being masked. Make sure to reduce the irq_num.
        let new_mask = in_b(SECONDARY_PIC_DATA) | (1 << (irq_num - NUM_IRQS));
        out_b(SECONDARY_PIC_DATA, new_mask);
    }
}

/// A function which disable the PICs alltogether. It should be called if the APIC is configured
/// and we're switching to it.
pub unsafe fn disable() {
    out_b(SECONDARY_PIC_DATA, DISABLE);
    out_b(PRIMARY_PIC_DATA, DISABLE);
}
