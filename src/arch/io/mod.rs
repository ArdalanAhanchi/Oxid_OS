pub mod textmode;
pub mod ps2_keyboard;

/// A function which calls end of interrupt for the IO related interrupts.
/// this is used to enable IO after a process exits before EOI.
pub unsafe fn end_of_interrupt() {
    crate::arch::interrupts::pic::end_of_interrupt(ps2_keyboard::IRQ_NUM); 
}
