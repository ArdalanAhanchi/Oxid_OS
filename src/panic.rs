//! A submodule which handles panics, and allows rust code to run in general. Proper panic handling
//! is crucial specially in debugging.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

use core::panic::PanicInfo;
use core::alloc::Layout;

#[lang = "eh_personality"]
#[no_mangle] 
pub extern fn rust_eh_personality() {}


/// The primary panic handler. By default, it runs the halt instruction, and print the panic
/// information to the console. More information can be found at:
/// https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
#[no_mangle]
pub extern fn oxid_panic(_info: &PanicInfo) -> ! {
    // Print the error message.
    oxid_err!("{}", _info);
    
    // Halt the system.
    loop{ unsafe { crate::arch::proc::halt(); }}
}

#[alloc_error_handler]
fn heap_allocation_err(_layout: Layout) -> ! {
    panic!("Error allocating memory");
}
