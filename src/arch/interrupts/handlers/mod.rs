//! A sub-module which is the primary high level handler of oxid os. It registers the basic handlers
//! and then allows dynamic registeration of new interrupt handlers.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

pub mod context;
pub mod exceptions;

use crate::proc::mutex::Mutex;
use crate::arch::interrupts::idt;
use crate::arch::proc;

/// Define a type for interrupt handlers (which is just a function which accepts a context).
type InterruptHandler = unsafe fn(*const context::Context);

/// Define an array of handlers protected with a mutex. Additionally, make the interrupt handlers 
/// optional (since not all of them might be registered). By default none of them are registered.
static mut HANDLERS: [Option<InterruptHandler>; idt::NUM_IDT_ENTRIES] = [None; idt::NUM_IDT_ENTRIES];

/// Define a basic mutex for the handlers.
static mut HANDLERS_MUTEX: Mutex = Mutex::new();

/// The main entry point for the interrupts. The interrupt number and the context is passed by the 
/// assembly code at arch/interrupts/idt/isr.asm (which actually calls this function). Based on the 
/// registered interrupts, it calls the corresponding high-level handler.
///
/// # Parameters
/// `int_num` : The interrupt number passed (0-255).
/// `info` : The context structure which determines what was going on before the interrupt.
#[no_mangle]
unsafe extern "sysv64" fn main_handler(int_num: u8, info: *const context::Context) {
    // Check if the handler is registered currently. Since we're not modifying anything in the 
    // handlers, we don't need to modify the mutex.
    match HANDLERS[int_num as usize] {
        // If it is, call the handler with the information.
        Some(handler) => handler(info),
        
        // Otherwise, just print an error message and halt.
        None => {
            oxid_err!("Unhandled interrupt #{} was recieved. Halting.", int_num);
            proc::halt();
        }
    };
}


/// A function which registers a handler and marks it a trap in the IDT. This means that interrupts
/// will not be masked, so new interrupts might be fired. Additionally, the execution will continue
/// in the current instruction (it basically retries). Please keep in mind that this will override
/// the previously registered handler.
///
/// # Parameters
/// `int_num` : The interrupt number (0-255).
/// `info` : The context structure which determines what was going on before the interrupt.
pub fn register_trap(int_num: u8, handler: InterruptHandler) {
    unsafe {
        // Lock the handlers so we don't get synchronization issues.
        HANDLERS_MUTEX.lock();
    
        // Check if there is one registered, if it is, show a warning message.
        if HANDLERS[int_num as usize] != None {
            //oxid_warn!("Interrupt {} already had a handler. Overriding the handler.", int_num);
        }
        
        // Then register the handler.
        HANDLERS[int_num as usize] = Some(handler);
        
        // Set the type to trap in IDT.
        super::idt::IDT[int_num as usize].set_trap_type();
        
        // Unlock the handlers so other processes can use it.
        HANDLERS_MUTEX.unlock();
    }   
}


/// A function which registers a handler and marks it an interrupt in the IDT. This means that 
/// interrupts will be masked, so new interrupts will be disabled. Additionally, the execution will 
/// continue in the next instruction (it does not retry). Please keep in mind that this will
/// override the previously registered handler.
///
/// # Parameters
/// `int_num` : The interrupt number (0-255).
/// `info` : The context structure which determines what was going on before the interrupt.
pub fn register_int(int_num: u8, handler: InterruptHandler) {
    unsafe {        
        // Lock the handlers so we don't get synchronization issues.
        HANDLERS_MUTEX.lock();
    
        // Check if there is one registered, if it is, show a warning message.
        if HANDLERS[int_num as usize] != None {
            //oxid_warn!("Interrupt {} already had a handler. Overriding the handler.", int_num);
        }
        
        // Then register the handler.
        HANDLERS[int_num as usize] = Some(handler);
        
        // Set the type to interrupt in IDT.
        super::idt::IDT[int_num as usize].set_interrupt_type();
        
        // Unlock the handlers so other processes can use it.
        HANDLERS_MUTEX.unlock();
    }
    
    
}


/// A function which unregisters an interrupt, and falls back to the default handler.
///
/// # Parameters
/// `int_num` : The interrupt number (0-255).
pub fn unregister(int_num: u8) {
     unsafe {
        // Lock the handlers so we don't get synchronization issues.
        HANDLERS_MUTEX.lock();
     
        // Set the handler at int_num to None.
        HANDLERS[int_num as usize] = None;
        
        // reset the type to interrupt in IDT.
        super::idt::IDT[int_num as usize].set_interrupt_type();
        
        // Unlock the handlers so other processes can use it.
        HANDLERS_MUTEX.unlock();
    }
}
