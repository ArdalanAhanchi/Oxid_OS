//! A basic program which simply clears the terminal.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

use crate::proc::process::Args;

/// The main function as specified by the system requirements.
///
/// # Parameters
/// `args` : The list of arguments.
pub extern "sysv64" fn main(_args: *const Args) {
    unsafe {
        // Simlpy clear the terminal.
        crate::console::CONSOLE.as_mut().expect("Console not initialized").clear();
    }  
}
