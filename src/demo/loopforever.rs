//! A basic program which goes into an infinite loop printing for ever (until interrupted).
//! For demonstration purposes.
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
        
        if full_args.len() > 1 {
            oxid_warn!("Launching loop.");
        
            // Print it forever.
            loop {
                oxid_print!("{}", full_args[1]);
            }
        } else {
            oxid_err!("Please pass in an argument.");
        }
    }  
}
