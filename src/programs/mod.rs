//! A module which includes some sample programs for demonstration and testing purposes.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

// Define the programs here.
pub mod clear;
pub mod echo;
pub mod poke;
pub mod loopforever;

use alloc::collections::btree_map::BTreeMap;

// The type for the main functions (defined by the scheduler).
type MainFn = extern "sysv64" fn(*const crate::proc::process::Args);

/// A map which holds the mapping between program names, and their main functions.
static mut PROGRAMS: Option<BTreeMap<&str, MainFn>> = None;

/// A function which initializes all the user programs into the programs tree. 
pub unsafe fn init() {
    // Initialize the programs.
    PROGRAMS = Some(BTreeMap::new());
    
    // Add the programs here with their names.
    PROGRAMS.as_mut().unwrap().insert("clear", clear::main);
    PROGRAMS.as_mut().unwrap().insert("echo", echo::main);
    PROGRAMS.as_mut().unwrap().insert("poke", poke::main);
    PROGRAMS.as_mut().unwrap().insert("loop", loopforever::main);
}

/// A function which returns the main function pointer to a given program with a specific name.
/// 
/// # Parameters
/// `name` : The name of the program registered in programs::init
///
/// # Returns
/// Some with a function pointer if it exsits, None otherwise.
pub fn get_main(name: &str) -> Option<MainFn> {
    unsafe {
        // Check if programs is initialized.
        match &mut PROGRAMS {
            // If it is, check if the key exists.
            Some(tree) => {
                // If it does, return it. Otherwise, return none.
                match tree.get(name) {
                    Some(func) => Some(*func),
                    None => None,
                }
            }, 
            
            None => None,
        }
    }
}
