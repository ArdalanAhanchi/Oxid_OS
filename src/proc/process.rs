//! A sub-module which defines processes and their properties.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

#![allow(dead_code)]

use alloc::string::String;
use alloc::vec::Vec;

/// The current status of the process. 
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
    pub args: *mut Args,            // Pointer to the arguments.
}

const ARGS_MAX: usize = 1024;

/// A structure for passing arguments to processes.
pub struct Args {
    buffer: [u8; ARGS_MAX],         // To hold the characters.
    len: usize,                     // To hold the number of them.
}

impl Args {
    /// A function that returns the list of arguments.
    ///
    /// # Returns
    /// A vector of strings which represent the arguments. Arg 0 is the
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
