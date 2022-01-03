//! A sub-module which implements a dynamic memory allocator for the kernel. It uses linked lists 
//! to manage the a usable memory region. It keeps a list of free and used memory regions to do so.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

mod heap_node;
mod heap_list;
mod heap_node_alloc;

extern crate alloc;

use crate::mem::region::Region;
use heap_list::HeapList;
use core::alloc::{GlobalAlloc, Layout};
use crate::proc::mutex::Mutex;

/// The static global allocator which will be used for kernel memory allocations. This is declared
/// global allocator so we can use the rust types. For more information, please look at:
/// https://doc.rust-lang.org/core/alloc/trait.GlobalAlloc.html
#[global_allocator]
static mut HEAP_ALLOC: HeapAlloc = HeapAlloc::new();

/// The mutex for the heap allocator to keep allocations memory safe.
static mut HEAP_ALLOC_MUTEX: Mutex = Mutex::new();

/// A structure which represents the heap allocator for oxid os. It utilizes two linked lists to 
/// hold the blocks and manage them.
struct HeapAlloc {
    free_list: Option<heap_list::HeapList>,         // List of all free regions (merged).
    used_list: Option<heap_list::HeapList>,         // List of all used regions.
    num_allocs: usize,                              // Keep the number of allocations.
}

impl HeapAlloc {
    /// A constructor which initializes an empty HeapAllocator (the lists aren't initialized).
    pub const fn new() -> Self {
        // Set both lists to None and return.
        HeapAlloc {
            free_list: None,
            used_list: None,
            num_allocs: 0,
        }
    }

    /// A method which initializes the default allocator based on a given metadata region and
    /// allocation region. Keep in mind that in both cases, only the virtual address space 
    /// will be occupied (frames aren't allocated until needed).
    ///
    /// # Parameters
    /// `meta_region` : The region which we're using for the metadata of the heap.
    /// `alloc_region` : The actual region where memory will be allocated at.
    pub unsafe fn init(&mut self, meta_region: &Region, alloc_region: &Region) {
        oxid_log!("Initializing the kernel heap.");
    
        // Divide the metadata region to two sections for free and used lists.
        let meta_region_free = Region::new_sized(meta_region.addr , meta_region.size / 2);
        let meta_region_used = 
            Region::new_sized(meta_region_free.end_addr(), meta_region_free.size);
        
        // Initialize the lists with their corresponding regions.
        self.free_list = Some(HeapList::new(&meta_region_free));
        self.used_list = Some(HeapList::new(&meta_region_used));
        
        // Add all the heap memory to the free list.
        self.free_list.as_mut().expect("List not initialized.").add(alloc_region, true)
            .expect("Could not add free region to the free list.");
    }
    
    /// A function which checks if a given layout can fit in a region (with needed padding). 
    ///
    /// # Parameters
    /// `free_region` : The region which we're checking for allocation.
    /// `layout` : The size and alignment which we're checking to see if it fits.
    ///
    /// # Returns
    /// Ok(offset) if it fits, Err if it doesn't fit. Offset is from the beginning of free region
    /// to where the memory will align correctly.
    fn can_fit(free_region: &Region, layout: &Layout) -> Result<usize, ()> {
        // Calculate the aligned start address based on the starting address of free_region.
        let aligned_start = crate::mem::align::align_higher(free_region.addr, layout.align());
        
        // Check if the aligned start and the the size based on the layout can fit.
        if aligned_start + layout.size() < free_region.end_addr() {
            Ok(aligned_start - free_region.addr)
        } else {
            Err(())
        }
    }
    
    /// The primary heap allocation code which will allocate a certain amount of memory based on 
    /// the passed layout (size, alignment), and it will map it to the virtual address space using
    /// the passed permissions. Interally, it will always allocate memory in page_size alignment. 
    /// However, the returned pointer might have a different alignment based on the layout given).
    ///
    /// # Parameters
    /// `layout` : The size and alignment that is requested for the returned ptr.
    /// `is_user` : True if the permissions are user accessible, False otherwise.
    /// `is_writable` : True if R/W, False if it's read-only.
    /// `is_no_exec` : True if not executable, False otherwise.
    ///
    /// # Returns
    /// The address of the allocated memory.
    #[inline]
    unsafe fn internal_alloc(&mut self, layout: &Layout, is_user: bool
        , is_writable: bool, is_no_exec: bool) -> *mut u8 {
        // Create a new layout with a page_size aligned size (just to ensure every allocation is 
        // at least one page long to avoid deallocation issues).
        let aligned_layout = Layout::from_size_align_unchecked(
            crate::mem::align::align_higher(layout.size(), crate::mem::vmm::PAGE_SIZE), 
            layout.align());
            
        // Unwrap the lists for future use.
        let free_list_uw = self.free_list.as_mut().expect("Heap alloc free list not valid.");
        let used_list_uw = self.used_list.as_mut().expect("Heap alloc used list not valid.");
        
        // Lock the allocator.
        HEAP_ALLOC_MUTEX.lock();
        
        // To store the address of allocated memory.
        let mut allocated_ptr: *mut u8 = 0 as *mut u8;
        
        // Go through the free list.
        for node_ptr in free_list_uw.into_iter() {
            // Get the current region.
            let curr_region = (*node_ptr).region;
            
            // Check if it can fit the requested aligned layout based on alignment.
            match HeapAlloc::can_fit(&curr_region, &aligned_layout) {
            
                // If it can fit, do the actual allocation.
                Ok(offset) => {           
                    // Remove the free region from the list of free items.
                    let free_region = free_list_uw.remove(node_ptr)
                        .expect("Could not remove region from the HeapList.");
                    
                    // Define the regions based on the start address and the free region.
                    let alloc_region = Region::new_sized(free_region.addr, aligned_layout.size());
                    let after_region = Region::new(alloc_region.end_addr(), free_region.end_addr());
                    
                    // Put the after region in the free list if needed.
                    if after_region.size > 0 {
                        free_list_uw.add(&after_region, true)
                            .expect("Could not add the after region to the free list.");
                    }
                    
                    // Put the allocated region into the used list.
                    used_list_uw.add(&alloc_region, false)
                        .expect("Could not add the allocated region to the used list.");
                
                    // Store the start address of the allocated region as the pointer.
                    // Add the offset to it to match the layout's alignment.
                    allocated_ptr = (alloc_region.addr + offset) as *mut u8;
                    
                    break;
                },
                
                // If not, don't do anything.
                Err(()) => (),
            }
        }
        
        self.num_allocs += 1;
        
        // Unlock the mutex since the critical section is over.
        HEAP_ALLOC_MUTEX.unlock(); 
        
        // Map the memory in VMM with the correct permissions.
        crate::mem::vmm::map_range(allocated_ptr as usize, aligned_layout.size()
            , is_user, is_writable, is_no_exec).expect("Could not map memory range");
        
        // Set the memory to all zeros.
        crate::olibc::memset::memset(allocated_ptr, 0, layout.size());
           
        allocated_ptr
    }
    
    /// The primary deallocation method (similar to free in Clib). It finds an allocation, and 
    /// deallocates it from the dynamic memory. Additionally, it will unmap the pages from the 
    /// page table.
    ///
    /// # Parameters
    /// `ptr` : The memory address (which we got from alloc), which we're freeing.
    #[inline]
    unsafe fn internal_dealloc(&mut self, ptr: *mut u8) {
        // Unwrap the lists for future use.
        let free_list_uw = self.free_list.as_mut().expect("Heap alloc free list not valid.");
        let used_list_uw = self.used_list.as_mut().expect("Heap alloc used list not valid.");
        
        // Align lower the pointer to match the page alignment.
        let aligned_ptr = crate::mem::align::align_lower(ptr as usize, crate::mem::vmm::PAGE_SIZE);
        
        // Lock the allocator.
        HEAP_ALLOC_MUTEX.lock();
        
        // Go through the used list.
        for node_ptr in used_list_uw.into_iter() {
            // Get the current region's address.
            let curr_addr = (*node_ptr).region.addr;
            
            // If the adderss is the same as the address pointed to by the ptr.
            if curr_addr == aligned_ptr {
                // Remove and get the region from the used list.
                let removed_region = used_list_uw.remove(node_ptr).expect("Count not remove ptr.");
                
                // Add it to the free list and merge if needed.
                free_list_uw.add(&removed_region, true).expect("Could not add ptr to free list.");
                
                // Unmap it from the vmm.
                crate::mem::vmm::unmap_range(removed_region.addr, removed_region.size)
                    .expect("Could not unmap memory range.");
                
                self.num_allocs -= 1;
                break;
            }
        }

        // Unlock the mutex since the critical section is over.
        HEAP_ALLOC_MUTEX.unlock();
    }
}

// TODO: Add synchronization.

/// A public wrapper for the internal alloc which always sets the alignment to page size, and 
/// can provide a familiar interface for kernel processes to allocate memory. Compared to the 
/// global allocator, it also provides a fine-grain control.
///
/// # Parameters
/// `size` : The number of bytes which will be allocated.
/// `is_user` : True if the permissions are user accessible, False otherwise.
/// `is_writable` : True if R/W, False if it's read-only.
/// `is_no_exec` : True if not executable, False otherwise.
///
/// # Returns
/// The address of the allocated memory.
pub unsafe fn kmalloc(size: usize, is_user: bool, is_writable: bool, 
    is_no_exec: bool) -> *mut u8 {
    // Define a new layout, and then call the internal allocator.
    let layout = Layout::from_size_align_unchecked(size, crate::mem::vmm::PAGE_SIZE);  
    HEAP_ALLOC.internal_alloc(&layout, is_user, is_writable, is_no_exec)
}

/// A wrapper for the internal dealloc method. This is only to provide a familiar interface
/// for kernel developers.
///
/// # Parameters
/// `ptr` : The memory address (which we got from alloc), which we're freeing.
pub unsafe fn kfree(ptr: *mut u8) {
    // Call the internal allocator.
    HEAP_ALLOC.internal_dealloc(ptr)
}

/// Implement global alloc so we can use rust standard types.
unsafe impl GlobalAlloc for HeapAlloc {
    /// The main entry for kernel heap allocation. It uses the standard rust allocation interface, 
    /// and can allocate memory with a certain size and alignment. This is a wrapper to make
    /// the internal implementation compatible with the rust's alloc library.
    ///
    /// # Parameters
    /// `layout` : The size and alignment (rust type).
    ///
    /// # Returns
    /// A pointer to the allocated memory region. 
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Get a mutable reference to self (odd syntax, not ideal but needed).
        let mut_self = &mut *(self as *const HeapAlloc as *mut HeapAlloc);
    
        // Since this is the kernel heap, set it to kernel mode, writable, and executable.
        mut_self.internal_alloc(&layout, false, true, false)
    }
    
    /// The main deallocation method which is similar to free in Clib. It uses the standard rust 
    /// deallocation interface. This is a wrapper to make the internal implementation compatible
    /// with the rust's alloc library.
    ///
    /// # Parameters
    /// `ptr` : The pointer for the location we're deallocaing.
    /// `_layout` : The size and alignment of the memory we're deallocating.
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // Get a mutable reference to self (odd syntax, not ideal but needed).
        let mut_self = &mut *(self as *const HeapAlloc as *mut HeapAlloc);
        
        // Simply call the internal implementation of dealloc.
        mut_self.internal_dealloc(ptr);
    }
}

/// A wrapper for the HeapAlloc::init method which initializes the global allocator.
///
/// # Parameters
/// `meta_region` : The region which we're using for the metadata of the heap.
/// `alloc_region` : The actual region where memory will be allocated at.
pub unsafe fn init(meta_region: &Region, alloc_region: &Region) {
    HEAP_ALLOC.init(meta_region, alloc_region)
}

// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        super::heap_node_alloc::test::run();
        super::heap_list::test::run();
    }
}
