//! A basic program which checks the memory contents of a passed location.
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
    
        // Parse the first argument and check the results.
        match full_args[1].trim().parse() {
            Ok(addr) => {
                oxid_println!();
                crate::debug::memview::hex_dump(addr, 15);
            },
            
            Err(_error) => {
                oxid_err!("Invalid address passed. Please check input.")
            }
        }
    }  
}
