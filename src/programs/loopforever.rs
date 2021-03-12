//! A basic program which goes into an infinite loop printing for ever (until interrupted).
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
        // Get the list of arguments.
        let full_args = (*args).get_args();
        
        oxid_warn!("Launching loop with {}", full_args[1]);
        
        // Print it forever.
        loop {
            //oxid_println!("{}", full_args[1]);
        }
    }  
}
