//! A module which includes all the architecture specific code. These codes abstract the underlying
//! hardware for the kernel to use. This is done to allow future architectures to be added.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

pub mod io;
pub mod proc;
pub mod interrupts;
pub mod mem;
pub mod registers;

/// A function which is called by the kernel main to intitialize the architecture specific code.
/// It might call other modules to initialize themselves if needed.
pub unsafe fn init() {
    oxid_log!("Initializing the architecture dependent code (x86_64).");
    
    // Initialize the PS2 keyboard.
    io::ps2_keyboard::init();
    
    // Initialize the processing code (TSS, etc.)
    proc::process::init();
}

// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        super::mem::test::run();
    }
}
