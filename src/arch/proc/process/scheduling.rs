//! A sub-module which is used to perform the low-level tasks needed for scheduling and context 
//! switching. It is used by the high-level scheduler.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

#![allow(dead_code)]

use crate::arch::interrupts::{handlers, pic};
use crate::arch::interrupts::handlers::context::Context;
use crate::proc::process::Args;

/// The IRQ number for the PIT timer in PIC (set initially by the system).
const IRQ_NUM: u8 = 0;

// The interrupt number based on the IRQ offset.
const INT_NUM: u8 = IRQ_NUM + crate::arch::interrupts::IRQ_OFFSET;

/// A function which initializes the PS2 keyboard driver, it registers the handler for the keyboard,
/// and enables the irq line for it.
pub fn init() {
    oxid_log!("Initializing the scheduleing module.");

    // Register the handler as an interrupt handler with the correct interrupt number.
    handlers::register_int(INT_NUM, schedule_process);
    
    // Enable the irq for this interrupt.
    unsafe { pic::enable_irq(IRQ_NUM); }
}

/// The low level scheduler which is directly called by the interrupt handler. It simply calls the 
/// high-level scheduler to perform the task. And sends an EOI to the PIC.
///
/// # Parameters
/// `context` : The passed context from the interrupt handling code.
#[inline]
unsafe fn schedule_process(context: *const Context) {
    // Call the high level handler with the context casted to a generic pointer.
    crate::proc::scheduler::schedule(context as *mut u8);
    
    // Send the EOI signal to the PIC.
    pic::end_of_interrupt(IRQ_NUM);  
}

/// A function which initializes a context to point to a given function (starting_point). It  
/// basically sets it's stack and starting point of the given function. Additionally, it sets the 
/// exit point of the function (return instruction), and passes argc and argv to the function based
/// on the sysv64 ABI.
///
/// # Parameters
/// `starting_point` : The function pointer which will be executed by this context.
/// `exit_point` : The function pointer where the programs jumps when finished execution.
/// `stack_start` : The starting addrss (high_addr) of the stack for this context.
/// `context_ptr` : The pointer to the context that we're initializing.
/// `args` : Pointer to the arguments.
pub unsafe fn init_context(starting_point: extern "sysv64" fn(*const Args), exit_point: fn()
    , stack_start: *mut u8, context_ptr: *mut u8, args: *const Args) {
    // Store a cast version for readability.
    let context = context_ptr as *mut Context;
    
    // Store the value of exit function's address, and then set the rsp to after it.
    let new_stack_start = (stack_start as usize) - core::mem::size_of::<usize>();
    *(new_stack_start as *mut usize) = exit_point as *const u8 as usize;
    (*context).orig_rsp = new_stack_start;
    
    // Set the intitial rip, and CS values to start at the correct instruction.
    (*context).rip = starting_point as *const u8 as usize;
    (*context).cs = crate::arch::registers::get_cs() as usize;
    
    // Set the RDI to args (first parameter based on sysv64 ABI).
    (*context).rdi = args as usize;
}

/// A function which sets a new context in the destination. It basically copies everything 
/// from the new context to the destionation excep the rflags.
///
/// # Parameters
/// `dst` : The address for the destination context (typically on the interrupt stack).
/// `new` : The address of the new context which is replacing the old one.
pub unsafe fn set_context(dst: *mut u8, new: *mut u8) {
    // Store the rflags from the previous context.
    let prev_eflags = (*(dst as *mut Context)).rflags;
    
    // Override the context at the destination.
    crate::olibc::memcpy::memcpy(dst, new, context_size());
    
    // Restore it's rflags.
    (*(dst as *mut Context)).rflags = prev_eflags;
}

/// A constant accessor for the size of the contexts. Used for implementing architecture independent
/// code without knowing the properties of the context.
///
/// # Returns
/// The size (in bytes) of the context in this architecture.
#[inline]
pub const fn context_size() -> usize {
    core::mem::size_of::<crate::arch::interrupts::handlers::context::Context>()
}
