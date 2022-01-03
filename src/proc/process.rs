//! A sub-module which defines processes and their properties.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;
use crate::arch::proc::process::scheduling;

/// Holds the size of the stack which will be allocated.
pub const STACK_SIZE: usize = 0x1000;

/// Holds the size of the context (from architecture dependent code).
pub const CONTEXT_SIZE: usize = scheduling::context_size();

/// Maximum size of arguments in bytes.
const ARGS_MAX: usize = 1024;

/// The current status of the process.
#[derive(PartialEq, Eq)] 
pub enum ProcessStatus {
    Started,                    // Started execution.
    Exited,                     // Finished execution.
}

/// Process control block. Holds all the information about a process.
pub struct PCB {
    pub pid: usize,                 // The process ID.
    pub name: String,               // Name of the process.
    pub status: ProcessStatus,      // Current status (Started, Exited, Waiting, etc.).
    pub stack_end: *mut u8,         // The pointer to the process stack end (low addr).
    pub context: *mut u8,           // The pointer to process context.
    pub args: Args,                 // Pointer to the arguments.
    pub prev: *mut PCB,             // Pointer to the previous process.
    pub next: *mut PCB,             // Pointer to the next process.
}

impl PCB {
    /// Default constructor which allocates a new PCB and initializes the 
    /// required fields as needed. It also requires a previous and next 
    /// pointers to be set explicitly since they are used by the scheduler.
    ///
    /// # Parameters
    /// `pid` : Process ID that is used for this PCB.
    /// `name` : Name of the process used for user identification.
    /// `prev` : The PCB that is scheduled before this one.
    /// `next` : The PCB that will be scheduled after this one.
    ///
    /// # Returns
    /// A pointer to the allocated process control block.
    pub unsafe fn alloc(pid: usize, name: &str, 
        prev: *mut PCB, next: *mut PCB) -> *mut PCB {
        // Allocate memory for a new PCB.
        let mut pcb: *mut PCB = crate::mem::dyn_alloc::kmalloc(
            core::mem::size_of::<PCB>(), 
            false, true, false) as *mut PCB;
    
        // Initialize all the fields and allocate memory as needed.
        (*pcb).pid = pid;
        (*pcb).name = String::from(name);
        (*pcb).status = ProcessStatus::Started;
        (*pcb).stack_end = crate::mem::dyn_alloc::kmalloc(STACK_SIZE, 
            false, true, false);
        (*pcb).context = crate::mem::dyn_alloc::kmalloc(CONTEXT_SIZE, 
            false, true, false);
        (*pcb).args = Args::new();
        (*pcb).prev = prev;
        (*pcb).next = next;
        
        return pcb;
    }
    
    /// Destructor which deletes a given PCB and deallocates the stack and the 
    /// context as it was initialized previously. 
    ///
    /// # Parameters
    /// `pcb` : A pointer to the process control block to deallocate.
    pub unsafe fn free(pcb: *mut PCB) {
        // TODO: Find out why freeing causes a problem and avoid leak.
        //crate::mem::dyn_alloc::kfree((*pcb).stack_end);
        crate::mem::dyn_alloc::kfree((*pcb).context);        
        crate::mem::dyn_alloc::kfree(pcb as *mut u8);
    }
}

/// A structure for passing arguments to processes.
#[derive(Copy, Clone)]
pub struct Args {
    buffer: [u8; ARGS_MAX],         // To hold the characters.
    len: usize,                     // To hold the number of them.
}

impl Args {
    /// A function that returns the list of arguments.
    ///
    /// # Returns
    /// A vector of strings which represent the arguments.
    pub fn get_args(&self) -> Vec<String> {
        let to_ret = String::from(core::str::from_utf8(&self.buffer[0..self.len]).unwrap());
        to_ret.split(' ').map(|st| String::from(st)).collect()
    }
    
    /// A function that saves the list of arguments.
    ///
    /// # Parameters
    /// `args` : All the arguments to be added.
    pub fn set_args(&mut self, args: &str) {
        for ch in args.bytes() {
            self.buffer[self.len] = ch;
            self.len += 1
        }
    }
    
    /// Default constructor which creates an empty args structure.
    pub const fn new() -> Self {
        Self {
           buffer: [0; ARGS_MAX],
           len: 0, 
        }
    }
}
