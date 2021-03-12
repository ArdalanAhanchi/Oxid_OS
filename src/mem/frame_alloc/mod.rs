//! A module which defines the basic frame allocator for the kernel. It utilizes a basic bit-map
//! and provides a standard allocate/deallocate functionality for the physical frames.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]                  // To allow partial usage without errors.

pub mod bitmap;                       // The actual bitmap allocator.
pub mod mem_info;                     // To get the kernel memory regions.      

use crate::proc::mutex::Mutex;        // For the safe access to the global frame allocator.
use crate::mem::region::Region;       // To get and utilize memory regions.
use crate::multiboot2::MultibootInfo; // To find out where to put the bit field, and memory size.
use bitmap::{BitMap, BitMapResult};   // For the actual allocation and deallocation.

pub const ALIGNMENT: usize = 0x1000;  // The alignment for the memory addresses (4K aligned).
pub const FRAME_SIZE: usize = 0x1000; // The size of the physical frame (same size as page size).

/// A static frame allocator which we can use to allocate globally.
static mut FRAME_ALLOCATOR: Option<BitMap> = None;

/// Create a new mutex for the frame allocator to allow safe access.
static mut FRAME_ALLOCATOR_MUTEX: Mutex = Mutex::new();

/// An enum which represents the result of a frame allocation.
pub enum FrameAllocResult {
    Ok(usize),          // When the allocation went as expected.
    Success,            // When a general operation was successful.
    Full,               // When the memory is full.
    InvalidAddr,        // When an invalid address was passed.
    Err,                // Other errors occured in allocating.
}

/// A function which initializes the bitmap frame allocator code. It calculates where the kernel
/// and memory end, and where the bitmap will be and initializes it.
///
/// # Parameters
/// `mb_info` : The multiboot information reference (already parsed).
pub unsafe fn init(mb_info: &MultibootInfo) {
    // Get the usable memory area after the kernel.
    let usable_region = mem_info::get_usable_region(mb_info);
    
    // Log the findings with the start and end address of the memory region.
    oxid_log!("Obtained the usable memory. kernel_end=0x{:x}, mem_end=0x{:x}"
        , usable_region.addr, usable_region.end_addr());

    // Initialize and set the global frame allocator.
    FRAME_ALLOCATOR = Some(bitmap::BitMap::new(&usable_region));
    oxid_log!("Initialized the frame allocator.");
}


/// A function which finds the first available free frame, and allocates it. It then returns the 
/// starting address of the frame. It provides thread safe access for the static bitmap which should
/// be accessible everywhere.
///
/// # Returns
/// FrameAlloc::Ok(frame_addr) if successful. 
/// FrameAlloc::Full if memory is full.
/// FrameAlloc::Err for other error occured.
pub fn alloc() -> FrameAllocResult {
    unsafe {
        // First, get a reference to the allocator and check if it's initialized.
        let allocator = match &mut (FRAME_ALLOCATOR) {
            Some(allocator) => allocator,
            None => panic!("Frame allocator not initialized."),
        };
    
        // Lock the mutex, allocate the frame, capture it's result, and unlock the mutex.
        FRAME_ALLOCATOR_MUTEX.lock();
        let alloc_result = allocator.alloc();
        FRAME_ALLOCATOR_MUTEX.unlock();
        
        // Check the allocation results, get frame number and return if it's full or error occured.
        let frame_num = match alloc_result {
            BitMapResult::Allocated(num) => num,
            BitMapResult::Full => return FrameAllocResult::Full,
            _ => return FrameAllocResult::Err,
        };
        
        // Get the starting frame address and check for errors.
        let frame_addr = match allocator.frame_to_addr(frame_num) {
            Ok(addr) => addr,
            Err(()) => panic!("Frame number error was not expected during translation."),
        };
                      
        //oxid_log!("Frame allocated at 0x{:x}", frame_addr);
    
        // If we get here, everything went as expected, return the calculated address.
        FrameAllocResult::Ok(frame_addr)
    }
}

/// A function which allocates a specified frame with an address. This function allows the kernel to 
/// not override parts of the memory which are used for other purposes (mapped otherwise). It is 
/// also thread-safe.
///
/// # Parameters
/// `physical_addr` : The physical address within the frame which we want to mark used.
///
/// # Returns
/// FrameAlloc::Success if successful. frame_addr is the starting address of the frame. 
/// FrameAlloc::InvalidAddr if the passed address is out of range.
/// FrameAlloc::Err if the frame is used or other error occured.
pub fn alloc_frame(physical_addr: usize) -> FrameAllocResult {
    unsafe {
        // First, get a reference to the allocator and check if it's initialized.
        let allocator = match &mut (FRAME_ALLOCATOR) {
            Some(allocator) => allocator,
            None => panic!("Frame allocator not initialized."),
        };
    
        // Get the frame number and check for invalid memory range.
        let frame_num = match allocator.addr_to_frame(physical_addr) {
            Ok(num) => num,
            Err(()) => return FrameAllocResult::InvalidAddr,
        };
        
        // Lock the mutex, allocate the frame, capture it's result, and unlock the mutex.
        FRAME_ALLOCATOR_MUTEX.lock();
        let alloc_result = allocator.alloc_frame_num(frame_num);
        FRAME_ALLOCATOR_MUTEX.unlock();
        
        // Check the allocation results.
        match alloc_result {
            BitMapResult::AlreadyUsed => return FrameAllocResult::Err,
            BitMapResult::InvalidFrameNum => panic!("Frame number error was not expected."),
            _ => (),
        }
        
        // Get the starting frame address and check for errors.
        let frame_addr = match allocator.frame_to_addr(frame_num) {
            Ok(addr) => addr,
            Err(()) => panic!("Frame number error was not expected during translation."),
        };
    
        // If we get here, everything went as expected, return the calculated address.
        FrameAllocResult::Ok(frame_addr)
    }
}

/// A function which deallocates a specified frame with an address. It is also thread-safe.
///
/// # Parameters
/// `physical_addr` : The physical address within the frame which we want to deallocate.
///
/// # Returns
/// FrameAlloc::Success if the frame was successfully deallocated or if it was unused.
/// FrameAlloc::InvalidAddr if the passed address is out of range.
/// FrameAlloc::Err if any other error occured.
pub fn dealloc(physical_addr: usize) -> FrameAllocResult {
    unsafe {
        // First, get a reference to the allocator and check if it's initialized.
        let allocator = match &mut (FRAME_ALLOCATOR) {
            Some(allocator) => allocator,
            None => panic!("Frame allocator not initialized."),
        };
    
        // Get the frame number and check for invalid memory range.
        let frame_num = match allocator.addr_to_frame(physical_addr) {
            Ok(num) => num,
            Err(()) => return FrameAllocResult::InvalidAddr,
        };
        
        // Lock the mutex, deallocate the frame, and unlock the mutex.
        FRAME_ALLOCATOR_MUTEX.lock();
        allocator.dealloc(frame_num);
        FRAME_ALLOCATOR_MUTEX.unlock();

        // If we get here, everything went as expected, return success.
        FrameAllocResult::Success
    }
}

/// A function which calculates the region which is mappable by this bitmap. It starts at the 
/// end of the bitmap (aligned), and ends at the end of the last frame. 
///
/// # Returns
/// A region which represents the usable memory area.
pub fn get_mappable_region() -> Region {
    unsafe {
        // First, get a reference to the allocator and check if it's initialized.
        let allocator = match & (FRAME_ALLOCATOR) {
            Some(allocator) => allocator,
            None => panic!("Frame allocator not initialized."),
        };
        
        // Then get the value from the bitmap and return it.
        allocator.get_mappable_region()
    }
}

// Unit Tests **************************************************************************************



/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        super::bitmap::test::run();
    }
}
