/// A sub-module which provides a basic driver for a PS2 keyboard. It registers an interrupt handler
/// for it, enables the IRQ, and gets a keycode when a key is pressed. It then calls the 
/// architecture independent code, and sends an end of interrupt to the PIC.
/// TODO: Check if ps2 keyboards are supported, and use APIC instead of PIC.
///
/// `Author` : Ardalan Ahanchi
/// `Date` : Feb 2021

use crate::arch::interrupts::{handlers, pic};
use crate::io::keyboard;

/// The IRQ number for the PS2 keyboard in PIC (set initially by the system).
const IRQ_NUM: u8 = 1;

// The interrupt number based on the IRQ offset.
const INT_NUM: u8 = IRQ_NUM + crate::arch::interrupts::IRQ_OFFSET;

// The port for the keyboard (to read keys from).
const KEYBOARD_IO_PORT: u16 = 0x60;

/// A function which initializes the PS2 keyboard driver, it registers the handler for the keyboard,
/// and enables the irq line for it.
pub fn init() {
    oxid_log!("Initializing the PS2 keyboard");

    // Register the handler as an interrupt handler with the correct interrupt number.
    handlers::register_int(INT_NUM, handle);
    
    // Enable the irq for this interrupt.
    unsafe { pic::enable_irq(IRQ_NUM); }
}

/// An interrupt handler for the keyboard interrupts. It gets the key-code, calls the high-level 
/// architecture independent code with the key code, and sends an end of interrupt to the PIC.
///
/// # Parameters
/// `info` : The context before the interrupt happended (registers, error code, etc.). 
pub fn handle(_info: *const handlers::context::Context) {
    unsafe { 
        // Get the keycode recieved from port 0x60.
        let key_code: u8 = pic::in_b(KEYBOARD_IO_PORT);
        
        // Translate the key code and get an event.
        let kb_event: keyboard::Event = keyboard::ps2::set_1::translate(key_code);
        
        // Call the event handler of the keyboard with the event.
        keyboard::handle_event(&kb_event);
        
        // Send eoi to the PIC so it can continue.
        pic::end_of_interrupt(IRQ_NUM); 
    }
}
