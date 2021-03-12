//! A basic program which echoes what was written in the terminal.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

use crate::proc::process::Args;

/// The main function as specified by the system requirements.
///
/// # Parameters
/// `args` : The list of arguments.
pub extern "sysv64" fn main(args: *const Args) {
    unsafe {
        let full_args = (*args).get_args();
    
        // Add a new line and print all arguments expect the command.
        oxid_println!("");
        for txt in &full_args[1..] {
            oxid_print!("{} ", txt);
        }
    }  
}
