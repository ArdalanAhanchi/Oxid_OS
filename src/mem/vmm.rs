//! A module which is the virtual memory management, and includes the initialization code to get the
//! page tables properly set-up. It should be called after initialization of the frame allocator.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

use crate::arch::mem::page_tables::PageTables;
use crate::mem::frame_alloc::FrameAllocResult;

/// The size of each virtual page (same as the frame size).
pub const PAGE_SIZE: usize = super::frame_alloc::FRAME_SIZE;

/// The amount of memory which was identity mapped by the bootstrap code.
const BOOT_ID_MAPPED_SIZE: usize = 0x40000000;

/// Holds the address and information of the kernel page table.
static mut KERNEL_PAGE_TABLE: PageTables = PageTables::new();

/// A function which initializes the bitmap memory section of the kernel. It initializes the frame 
/// allocator, page tables, and identity maps the correct amount of memory.
///
/// # Parameters
/// `id_map_end` : The end of the identity mapped area after the kernel and frame allocator bitmap.
pub unsafe fn init(id_map_end: usize) {
    // Setup the kernel page table.
    KERNEL_PAGE_TABLE.setup_kernel_pagetable();
    
    oxid_log!("Kernel page table was set-up. Identity mapping from 0x0 to 0x{:x}", id_map_end);

    // Identity map everything up to the id_map_end.
    if lazy_identity_map_range(0, id_map_end, false, true, false).is_err() {
        panic!("Error identity mapping.");
    }
    
    // Unmap every address which was previously identity mapped (by bootstrap code).
    if lazy_unmap_range(id_map_end, BOOT_ID_MAPPED_SIZE - id_map_end).is_err() {
        panic!("Error unmapping the extra kernel identity mapped area.");
    };

    // Actually load the page table into the system (also flushes the TLB).
    KERNEL_PAGE_TABLE.load();
}

/// A wrapper for the architecture dependent map function. This is done to abstract the hardware 
/// implementation of the map function. It maps a given page address (starting address) to a given
/// frame address. It additionally sets the required permissions and creates new tables if needed.
/// This function will not allocate a frame. Thus it is called "lazy".
///
/// # Parameters
/// `page_addr` : The starting address of the wanted page.
/// `frame_addr` : The starting address of the frame we're mapping the page to.
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn lazy_map(page_addr: usize, frame_addr: usize, is_user: bool, is_writable: bool,
    is_no_exec: bool) -> Result<(), ()> {
    // Simply call the architecture dependent code.
    PageTables::map(page_addr, frame_addr, is_user, is_writable, is_no_exec)
}

/// A wrapper for the lazy_map function which performs it with a certain range of memory. It is very 
/// similar to it. However, it also accepts a size. Please keep in mind that this function should
/// never be used since there is typically no guarantee that the frames are continous.
///
/// # Parameters
/// `page_addr` : The starting address of the wanted page.
/// `frame_addr` : The starting frame of the wanted page.
/// `size` : The number of bytes starting from page_addr. 
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn lazy_map_range(page_addr: usize, frame_addr: usize, size: usize, is_user: bool, 
    is_writable: bool, is_no_exec: bool) -> Result<(), ()> {
    // Go through every page and map it. If error occurs, return the Err.
    for page_num in 0..get_num_pages(size) {
        // Calculate the offset from the page and frame addresses.
        let offset = page_num * PAGE_SIZE;
        
        // Do the actual opreation with the offsets.
        lazy_map(page_addr + offset, frame_addr + offset, is_user, is_writable, is_no_exec)?;
    }
    
    Ok(())
}

/// A wrapper for the architecture dependent unmap function. This is done to abstract the hardware 
/// implementation of the unmap function. It unmaps a given page address. It resets the lowest level
/// page table entry for this specific page and "frees" it for future use. This function will not 
/// deallocate the frame corresponding to page addr, thus it is called "lazy".
///
/// # Parameters
/// `page_addr` : The address of the page which we're unmapping.
///
/// # Returns
/// Ok if the given page was unmapped, Err if invalid address or non-existant page.
#[inline(always)]
pub unsafe fn lazy_unmap(page_addr: usize) -> Result<(), ()> { 
    // Simply call the architecture dependent code.
    PageTables::unmap(page_addr)
}

/// A wrapper for the laz_unmap function which performs it with a certain range of memory. It is very 
/// similar to it. However, it also accepts a size. It does not deallocate the frames from the frame
/// allocator.
///
/// # Parameters
/// `page_addr` : The address of the page which we're unmapping.
/// `size` : The number of bytes starting from page_addr. 
///
/// # Returns
/// Ok if the given pages were unmapped, Err if invalid address or non-existant page.
#[inline(always)]
pub unsafe fn lazy_unmap_range(page_addr: usize, size: usize) -> Result<(), ()> {
    // Go through every page and map it. If error occurs, return the Err.
    for page_num in 0..get_num_pages(size) {
        // Calculate the offset from the page and frame addresses.
        let offset = page_num * PAGE_SIZE;
        
        // Do the actual opreation with the offsets.
        lazy_unmap(page_addr + offset)?;
    }
    
    Ok(())
}

/// A wrapper for the map function which performs identity mapping for a certain physical 
/// address. It does not check for the memory range. So please make sure that the passed memory
/// is not out of range. It does no mark the frames as used in frame_alloc.
///
/// # Parameters
/// `frame_addr` : The starting address of the frame we're identity mapping.
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn lazy_identity_map(frame_addr: usize, is_user: bool, 
    is_writable: bool, is_no_exec: bool) -> Result<(), ()> {
    // Simply call the map function with only the frame address.
    lazy_map(frame_addr, frame_addr, is_user, is_writable, is_no_exec)
}

/// A wrapper for the identity mapping which performs it with a certain range of memory. It is 
/// very similar to it. However, it also accepts a size. It does not mark the frame in the
/// frame allocator at all.
///
/// # Parameters
/// `frame_addr` : The starting address of the frame we're identity mapping.
/// `size` : The number of bytes starting from page_addr. 
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn lazy_identity_map_range(frame_addr: usize, size: usize, is_user: bool, 
    is_writable: bool, is_no_exec: bool) -> Result<(), ()> {
    // Go through every page and map it. If error occurs, return the Err.
    for page_num in 0..get_num_pages(size) {
        // Calculate the offset from the page and frame addresses.
        let offset = page_num * PAGE_SIZE;
        
        // Do the actual opreation with the offsets.
        lazy_identity_map(frame_addr + offset, is_user, is_writable, is_no_exec)?;
    }
    
    Ok(())
}

/// A function which allocates a frame, and then maps it in pt. It maps a given page address 
/// (starting address) to a frame obtained from frame_alloc. It additionally sets the required 
/// permissions and creates new tables if needed.
///
/// # Parameters
/// `page_addr` : The starting address of the wanted page.
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn map(page_addr: usize, is_user: bool, is_writable: bool,
    is_no_exec: bool) -> Result<(), ()> {
    // Allocate a new frame, and check the results.
    let new_frame_addr = match crate::mem::frame_alloc::alloc() {
        FrameAllocResult::Ok(addr) => addr,
        FrameAllocResult::Full => panic!("Could not allocate frame. All frames are full."),
        _ => panic!("Could not allocate frame. Unknown error."),
    };

    // Map the allocated frame to the page address in the kernel page table.
    #[cfg(feature = "show-page-faults")]
    oxid_warn!("Mapping page 0x{:x} to frame 0x{:x}", page_addr, new_frame_addr);
    
    // Simply call the architecture dependent code with the new frame address.
    PageTables::map(page_addr, new_frame_addr, is_user, is_writable, is_no_exec)
}

/// A wrapper for the map function which performs it with a certain range of memory. It is very 
/// similar to it. However, it also accepts a size.
///
/// # Parameters
/// `page_addr` : The starting address of the wanted page.
/// `size` : The number of bytes starting from page_addr. 
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn map_range(page_addr: usize, size: usize, is_user: bool, 
    is_writable: bool, is_no_exec: bool) -> Result<(), ()> { 
    // Go through every page and map it. If error occurs, return the Err.
    for page_num in 0..get_num_pages(size) {
        // Calculate the offset from the page and frame addresses.
        let offset = page_num * PAGE_SIZE;
        
        // Do the actual opreation with the offsets.
        map(page_addr + offset, is_user, is_writable, is_no_exec)?;
    }
    
    Ok(())
}

/// A function which unmaps a given page. It first obtains it's physical address, and then 
/// unmaps it from the page table.
///
/// # Parameters
/// `page_addr` : The address of the page which we're unmapping.
///
/// # Returns
/// Ok if the given page was unmapped, Err if invalid address or non-existant page.
#[inline(always)]
pub unsafe fn unmap(page_addr: usize) -> Result<(), ()> {
    // Get the physical address first.
    let physical_addr = virt_to_phys(page_addr)?;
    
    // Deallocate it from the frame allocator.
    crate::mem::frame_alloc::dealloc(physical_addr);

    // Unmap it from the page table.
    PageTables::unmap(page_addr)
}

/// A wrapper for the unmap function which performs it with a certain range of memory. It is very 
/// similar to it. However, it also accepts a size.
///
/// # Parameters
/// `page_addr` : The address of the page which we're unmapping.
/// `size` : The number of bytes starting from page_addr. 
///
/// # Returns
/// Ok if the given pages were unmapped, Err if invalid address or non-existant page.
#[inline(always)]
pub unsafe fn unmap_range(page_addr: usize, size: usize) -> Result<(), ()> {
    // Go through every page and map it. If error occurs, return the Err.
    for page_num in 0..get_num_pages(size) {
        // Calculate the offset from the page and frame addresses.
        let offset = page_num * PAGE_SIZE;
        
        // Do the actual opreation with the offsets.
        unmap(page_addr + offset)?;
    }
    
    Ok(())
}

/// Function identity mapping for a certain physical address. It also marks it used in the frame 
/// allocator. If it fails, it means that the caller has to make sure the pages aren't being used
/// elsewhere.
///
/// # Parameters
/// `frame_addr` : The starting address of the frame we're identity mapping.
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn identity_map(frame_addr: usize, is_user: bool, 
    is_writable: bool, is_no_exec: bool) -> Result<(), ()> {
    // Mark the frame used in the frame allocator. Return if err.
    match crate::mem::frame_alloc::alloc_frame(frame_addr) {
        FrameAllocResult::Success => {
            // Simply call the lazy map function with only the frame address.
            lazy_map(frame_addr, frame_addr, is_user, is_writable, is_no_exec)
        }
        
        _ => Err(())
    }
}

/// A wrapper for the identity mapping which performs it with a certain range of memory. It is 
/// very similar to it. However, it also accepts a size.
///
/// # Parameters
/// `frame_addr` : The starting address of the frame we're identity mapping.
/// `size` : The number of bytes starting from page_addr. 
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// Ok if everything went as expected, Err otherwise.
#[inline(always)]
pub unsafe fn identity_map_range(frame_addr: usize, size: usize, is_user: bool, 
    is_writable: bool, is_no_exec: bool) -> Result<(), ()> {
    // Go through every page and map it. If error occurs, return the Err.
    for page_num in 0..get_num_pages(size) {
        // Calculate the offset from the page and frame addresses.
        let offset = page_num * PAGE_SIZE;
        
        // Do the actual opreation with the offsets.
        identity_map(frame_addr + offset, is_user, is_writable, is_no_exec)?;
    }
    
    Ok(())
}

/// A wrapper for the architecture dependent virt_to_phys function. It translates a given virtual 
/// address to it's corresponding physical address based on the currently loaded page table. It will
/// return an Err if the page table is not set-up or if the address is not currently mapped.
///
/// # Parameters
/// `page_addr` : The address of the page which we're translating.
///
/// # Returns
/// Ok(frame_addr) if everything went as expected, Err otherwise.
#[inline(always)]
pub fn virt_to_phys(page_addr: usize) -> Result<usize, ()> {
    // Simply call the architecture dependent code.
    PageTables::virt_to_phys(page_addr)
}

/// A function which calculates the number of pages needed to map a certain amount of memory.
///
/// # Parameters
/// `size` : The number of bytes we're mapping.
///
/// # Returns
/// The number of pages which are required to represent this memory.
#[inline]
pub fn get_num_pages(size: usize) -> usize {
    if size < PAGE_SIZE {       // If we have less than one page_size, we need at least one page.
        1 
    } else {
        size / PAGE_SIZE        // Otherwise, divide and return the results.
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
        // Some examples to test paging and the handling of page faults.
        /* 
        unsafe {
            
            use crate::arch::mem::page_tables::PageTables;
            
            // These tests break the kernel, and they will cause issues when page faults are checked 
            // by the kernel (since they are accessing memory where they shouldn't). 
            // Thus, they are disabled by default.
            
            // Cause a page fault, then read from it to ensure it was written to.         
            *(0xFBCDEFABC as *mut usize) = 123456;
            assert_eq!(123456, *(0xFBCDEFABC as *mut usize));
            
            // Unmap the page which was just mapped by the page fault.
            PageTables::unmap(0xFBCDEFABC).expect("Unmapping test failed.");
            
            // Translate a page which was identity mapped by the kernel and check the results.
            let physical_addr = PageTables::translate(0x6060).expect("Translation failed.");
            assert_eq!(0x6060, physical_addr);
            
            // Then try to translate an unmapped page which should result in an error.
            if PageTables::translate(0xFBCDEFABC).is_ok() {
                panic!("Translation test failed. The page should be unmapped.");
            };
            
        };
        */
    }
}


