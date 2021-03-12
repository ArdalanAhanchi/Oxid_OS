//! A file that handles the Non-maskable interrupt exception. The handle function is called 
//! automatically from the main interrupt handler. All the exception handlers are registered in the
//! interrupts::handlers::exceptions::init function.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

use crate::arch::interrupts::handlers::context;
use crate::arch::proc;

/// A function which is registered to handle the Non-maskable interrupt exception. Since it is an
/// exception, it should not resume execution until the problem is solved. By default, this 
/// will display an error message corresponding to the error and halt.
///
/// # Parameters
/// `info` : The context before the interrupt happended (registers, error code, etc.).
pub fn handle(_info: *const context::Context) {
    oxid_err!("Non-maskable interrupt exception recieved. Halting the system.");
    unsafe { proc::halt(); }
}
