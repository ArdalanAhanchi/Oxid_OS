#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![no_std]			            // There is no standard library in this enviornment.


#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![allow(unused_parens)]

mod console;
mod olibc;
mod arch;
mod io;
mod multiboot2;
mod proc;
mod debug;
mod mem;
mod panic;
mod programs;

extern crate alloc;

#[no_mangle]		// Done to avoid name mangling (since it's called from ASM).
pub unsafe extern "sysv64" fn kernel_main(multiboot_info: usize) {
    // Initialize the console so we can write to it globally.
    console::init();
    oxid_log!("Initialized the console.");
    
    // Parse the multiboot information.
    let mb_info = multiboot2::MultibootInfo::parse(multiboot_info).unwrap();
    oxid_log!("Parsed the multiboot2 information header.");
    
    // Initialize the interrupt handling code.
    arch::interrupts::init();
    
    // Initialize the memory code.
    mem::init(&mb_info);

    // Initialize the rest of what needs to be initialized on the hardware side.
    arch::init();
    
    // Initialize the scheduling code and run the scheduler.
    arch::proc::process::scheduling::init();
    proc::scheduler::init();
    
    // Initialize the interactive terminal.
    io::term::init();
    
    // Initialize the demonstration programs.
    programs::init();
    
    // Run the unit tests if the unit-test feature is set.
    #[cfg(feature = "unit-test")]
    test::run();
    
    // If we ever get here, halt forever.
    loop{ crate::arch::proc::halt(); }
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
        super::arch::test::run();
    }
}
