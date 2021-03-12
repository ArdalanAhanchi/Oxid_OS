//! A module which provides memory management functionality. Oxid OS only supports virtual memory,
//! so this module provides physical frame management, virtual memory management, and heap
//! allocation for both the kernel and usermodes.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

pub mod dyn_alloc;
pub mod frame_alloc;
pub mod align;
pub mod bitwise;
pub mod vmm;
pub mod page_fault;
pub mod region;
pub mod map;

use crate::multiboot2::MultibootInfo;
use region::Region;

/// A function which initializes the bitmap memory section of the kernel. It initializes the frame 
/// allocator, page tables, and identity maps the correct amount of memory.
///
/// # Parameters
/// `mb_info` : The multiboot information reference (already parsed).
pub unsafe fn init(mb_info: &MultibootInfo) {
    // First initialize the frame allocator using the multiboot information.
    frame_alloc::init(&mb_info);
    
    // Get the mappable physical memory from the frame allocator and initialize the KERNEL_END_ADDR.
    map::KERNEL_END_ADDR = frame_alloc::get_mappable_region().addr;
    
    // Initialize the virtual mem manager and identity map everything up to the the usable region.
    vmm::init(map::KERNEL_END_ADDR);
    
    // Initialize the kernel dynamic memory allocator (heap).
    let metadata_mem = Region::new(map::KERNEL_END_ADDR, map::KERNEL_HEAP_METADATA_END_ADDR);
    let heap_mem = Region::new(map::KERNEL_HEAP_METADATA_END_ADDR, map::KERNEL_HEAP_END_ADDR);
    dyn_alloc::init(&metadata_mem, &heap_mem);
}

// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        super::align::test::run();
        super::frame_alloc::test::run();
        super::bitwise::test::run();
        super::vmm::test::run();
        super::dyn_alloc::test::run();
    }
}
