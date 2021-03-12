//! A file that handles the Page fault exception. The handle function is called automatically 
//! from the main interrupt handler. All the exception handlers are registered in the
//! interrupts::handlers::exceptions::init function.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

use crate::arch::interrupts::handlers::context;
use crate::mem::bitwise::BitWise;

/// A function which is registered to handle the Page fault exception. Since it is an
/// exception, it should not resume execution until the problem is solved. This will gather
/// the required information to handle the page fault, and then calls the high-level
/// page fault handler (architecture independent).
///
/// # Parameters
/// `info` : The context before the interrupt happended (registers, error code, etc.).
pub fn handle(info: *const context::Context) {
    unsafe {
        // Get the address of the page which cause the fault from the CR2 register, and clear 
        // out the properties bits (first 12 bits).
        let page_addr = crate::arch::registers::get_cr2() & (!0xFFF);
        
        // Get the error codes for the resulted page fault from the interrupt context passed here.
        // The encoding of these are explained at: https://wiki.osdev.org/Exceptions#Page_Fault
        let present = (*info).err_code.is_set(0);
        let write = (*info).err_code.is_set(1);
        let user = (*info).err_code.is_set(2);
        let no_exec = (*info).err_code.is_set(4);
        
        // Call the high-level handler with the gathered error codes and address.
        crate::mem::page_fault::page_fault(page_addr, present, write, user, no_exec);
    }
}
