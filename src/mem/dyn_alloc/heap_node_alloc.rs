//! A sub-module which represents an allocator for each of the heap nodes. It allows allocation and 
//! freeing of the nodes. It is used to implement the linked list for the heap.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

use super::heap_node::HeapNode;                // To represent nodes.
use crate::mem::region::Region;                // For initialization.

/// The size of the nodes when wrapped in an option (which is the type stored).
const OPT_NODE_SIZE: usize = core::mem::size_of::<Option<HeapNode>>();

/// A structure which represents a basic allocator for HeapNodes. This will be used in the
/// implementation of the linked list to hold the allocators.
#[derive(Debug)]
pub struct HeapNodeAlloc {
    start_addr: usize,                         // The starting address of first in the list.
    curr_count: usize,                         // The number of elements in the list (currently).
    used_count: usize,                         // Number of used (not None) elements in the list.
    max_count: usize,                          // Most elements supprted in this list.
}

impl HeapNodeAlloc {
    /// A default constructor which constructs an allocator which can be used to allocate nodes.
    ///
    /// # Parameters
    /// `mem_region` : The start addr and size of the memory which will be used by this list.
    pub unsafe fn new(mem_region: &Region) -> Self {
        HeapNodeAlloc {
            start_addr: mem_region.addr,
            curr_count: 0,
            used_count: 0,
            max_count: mem_region.size / OPT_NODE_SIZE,
        }
    }
    
    /// The main allocator which allocates an empty heap node, and returns a pointer to it.
    ///
    /// # Returns
    /// Ok(ptr_to_node) if successful, Err if the list is full or other error occured.
    pub unsafe fn alloc(&mut self) -> Result<*mut HeapNode, ()> {
        // Check if the allocator is full.
        if self.used_count == self.max_count {
            return Err(());
        }
        
        // Hold an index for the element which we want to add.
        let mut idx: usize = 0;
        
        // Check if we need to increase the curr_count and add an element at the end.
        if self.curr_count == self.used_count {
            // Set the idx to the last one.
            idx = self.curr_count;
        
            // Clear out the memory for the new section.
            crate::olibc::memset::memset(
                (self.start_addr + idx * OPT_NODE_SIZE) as *mut u8, 0, OPT_NODE_SIZE);
            
            // Increase the count.
            self.curr_count += 1;
            
        // If we get here, we had a previous cell which we can re-use.
        } else {
            // Find the first cell which is None, and set the idx to it's idx.
            for i in 0..self.curr_count {
                // If we found an empty one, store it's index and break the loop.
                if self[i].is_none() {
                    idx = i;
                    break;
                }
            }
        }
        
        // Create a default node (everything set to 0), and set it's index for the future.
        let mut new_node = HeapNode::default();
        new_node.list_idx = idx;
        
        // Create a new default node (Everything is 0), and set it to the correct idx.
        self[new_node.list_idx] = Some(new_node);
        
        // Increase the number of used nodes.
        self.used_count += 1;
        
        // Get a reference to the node directly, and return a pointer to it.
        Ok(self[new_node.list_idx].as_mut().expect("Fatal error in Node alloc") as *mut HeapNode)
    }
    
    /// The main deallocator for HeapNodes. It relies on the list_idx field of the node. It sets 
    /// the correct index to None and "frees" it.
    ///
    /// # Returns
    /// Ok if it was successfully freed, Err if the ptr was not valid.
    pub fn free(&mut self, node_ptr: *mut HeapNode) -> Result<(), ()> {
        // To check the address constraints.
        let node_addr: usize = node_ptr as usize;
        
        // Check if the pointer is out of the acceptable address range. Or if the address is not 
        // aligned correctly (right when HeapNodes should start).
        if (node_addr < self.start_addr) 
            || (node_addr > self.start_addr + (self.curr_count * OPT_NODE_SIZE)) 
            || ((node_addr - self.start_addr) % OPT_NODE_SIZE != 0) {
            return Err(())
        }
        
        // Get the index of the node we're freeing.
        let idx: usize = unsafe { (*node_ptr).list_idx };
        
        // Set the node at the idx to None.
        self[idx] = None;
        
        // Decrease the number of used elements.
        self.used_count -= 1;
        
        // If the idx was the last one, decrease the number of current elements.
        if idx == (self.curr_count - 1) {
            self.curr_count -= 1;
        }
        
        // If we got here, everything went as expected.
        Ok(())
    }
    
    /// A method to get the current number of used nodes.
    ///
    /// # Returns
    /// The number of nodes which are active (allocated, not None).
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.used_count
    }
}

/// Implement the index trait to be able to access elements directly.
impl core::ops::Index<usize> for HeapNodeAlloc {
    type Output = Option<HeapNode>;
    
    /// A function which is an accessor for the array enclosed.
    #[inline(never)]
    fn index(&self, index: usize) -> &Option<HeapNode> {
        // Make sure the index is within range.
        assert!(index < self.curr_count);
    
        // Calculate the address of the wanted entry, and return a reference to it.
        let addr = self.start_addr + (OPT_NODE_SIZE * index);
        unsafe { &*(addr as *const Option<HeapNode>) }
    }
}

/// Implement the mutable index trait to be able to modify elements directly.
impl core::ops::IndexMut<usize> for HeapNodeAlloc {
    /// A function which is a mutable accessor for the array enclosed.
    #[inline(never)]
    fn index_mut(&mut self, index: usize) -> &mut Option<HeapNode> {
        // Make sure the index is within range.
        assert!(index < self.curr_count);
    
        // Calculate the address of the wanted entry, and return a reference to it.
        let addr = self.start_addr + (OPT_NODE_SIZE * index);
        unsafe { &mut *(addr as *mut Option<HeapNode>) }
    }
}


// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    use crate::mem::dyn_alloc::heap_node_alloc::HeapNodeAlloc;
    use crate::mem::dyn_alloc::heap_node::HeapNode;
    use crate::mem::region::Region;

    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        unsafe {
            // Create a region at the end of the kernel.
            const SIZE: usize = core::mem::size_of::<HeapNode>() * 3;
            let start_addr = crate::mem::map::KERNEL_END_ADDR;
            let reg = Region::new_sized(start_addr, SIZE);
        
            // Save the previously written data at address to restore it later.
            let prev_data: [u8; SIZE] = [0; SIZE];
            crate::olibc::memcpy::memcpy(prev_data.as_ptr() as *mut u8, start_addr as *const u8, SIZE);
            
            // First create an allocator.
            let mut alloc = HeapNodeAlloc::new(&reg);
            
            // Allocate some nodes and check if their addresses are correct.
            let node_1 = alloc.alloc().expect("Could not allocate node 1.");
            assert_eq!(node_1 as usize, start_addr);
            
            let node_2 = alloc.alloc().expect("Could not allocate node 2.");
            assert_eq!(node_2 as usize, start_addr + core::mem::size_of::<HeapNode>());
            
            let node_3 = alloc.alloc().expect("Could not allocate node 3.");
            assert_eq!(node_3 as usize, start_addr + core::mem::size_of::<HeapNode>() * 2);
            
            // Free the second node.
            alloc.free(node_2).expect("Could not deallocate node 2.");
            
            // Allocate a new node at capacity, check if the address is the same as node 2.
            let node_4 = alloc.alloc().expect("Could not allocate node 4.");
            assert_eq!(node_4 as usize, node_2 as usize);
            
            // Free the rest of the nodes.
            alloc.free(node_1).expect("Could not deallocate node 1.");
            alloc.free(node_3).expect("Could not deallocate node 3.");
            alloc.free(node_4).expect("Could not deallocate node 4.");
            
            // Make sure the table length is correct.
            assert_eq!(alloc.len(), 0);
            
            // Restore the data from the backup.
            crate::olibc::memcpy::memcpy(start_addr as *mut u8, prev_data.as_ptr(), SIZE);
        }
    }
}
