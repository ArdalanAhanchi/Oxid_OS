//! A module which includes all the architecture specific code for processing code. 
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

#![allow(dead_code)]        // To allow these to exist without actually calling them.

pub mod sync;
pub mod process;

/// A group of signature definitions to allow calling assembly code from rust.
/// it utilizes the System V AMD64 calling conventions to call the assembly code.
extern "sysv64" {
    /// A wrapper which executes the pause instruction from the x86 specifications. It is mainly
    /// used for implementing more efficient locking methodology for synchronization primitives.
    pub fn pause();
    
    /// A wrapper for the hlt instruction which simply puts the CPU in low power mode.
    pub fn halt();
}
