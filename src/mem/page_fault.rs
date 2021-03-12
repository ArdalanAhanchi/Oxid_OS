//! A file which includes the high-level page fault handler. The registration of the interrupt, 
//! gathering of the error codes and addresses, and calling this handler is done in the architecture
//! dependent portion of the kernel.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

/// A function which is called by the low-level page_fault handler. It checks the error codes passed
/// (such as the permissions and the context of the interrupt), and handles the fault accordingly.
/// If it's just a page that is not present, it allocates a frame of memory, and maps it to the 
/// page address which caused the fault in the first place.
///
/// # Parameters
/// `page_addr` : The virtual address of the page which caused the fault.
/// `present` : True if the page was already present in the table (page protection violation).
/// `write` : True if the operation that caused the fault was a write opertaion.
/// `user` : True if the operation was performed by a user (as opposed to the kernel).
/// `no_exec` : True if it was caused by an instruction fetch (when no exec is set in the table). 
pub unsafe fn page_fault(page_addr: usize, present: bool, _write: bool, user: bool, no_exec: bool) {
    #[cfg(feature = "show-page-faults")]
    oxid_warn!("Page fault (was_write={}) recieved for address 0x{:x}", _write, page_addr);
    
    // Check if the page was already present (violation).
    if present {
        panic!("Page protection violation occured. Can not handle page fault.");
    }
    
    // Check if an instruction fetch occured (where the ne bit was set).
    if no_exec {
        panic!("Tried to execute code (fetch instruction) without proper permissions.");
    }
    
    // Check if it was a user or kernel access.
    if user {
        panic!("Users can not allocate new frames.");
    } else {
        // Check if the kernel should be mapping pages here. Basically, the kernel can map pages 
        // using page faults if it's either in the area before the heap, or the area reserved for 
        // the page tables.
        if page_addr < super::map::KERNEL_HEAP_METADATA_END_ADDR 
            || (page_addr >= super::map::PAGE_TABLES_START_ADDR 
            && page_addr < super::map::PAGE_TABLES_END_ADDR) {
            // In such cases, we can map the page.
            crate::mem::vmm::map(page_addr, user, true, false)
                .expect("Could not map address during page fault.");
        } else {
            panic!("Invalid memory access. Please allocate the memory first.");
        }
        
    }    
}


