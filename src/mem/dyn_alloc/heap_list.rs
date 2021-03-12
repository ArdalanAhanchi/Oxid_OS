//! A sub-module which represents the linked list structure which is used internally for this 
//! allocator. It utilizes the heap_node_alloc to allocate and manage nodes. It additionally
//! implements an iterator for safe(er) access to the elements.
//! TODO: Optimizations, make the allocation more efficient, use a dual linked list maybe.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

#![allow(dead_code)]

use crate::mem::region::Region;                // To represent memory regions.
use super::heap_node::HeapNode;                // To represent nodes.
use super::heap_node_alloc::HeapNodeAlloc;     // To allocate nodes.

/// A structure which represents a basic allocator for HeapNodes. This will be used in the
/// implementation of the linked list to hold the allocators.
pub struct HeapList {
    node_alloc: HeapNodeAlloc,                 // To allocate new nodes to use.
    head: Option<*mut HeapNode>,               // The first node in the list.
}

impl HeapList {
    /// A default constructor which constructs a  which can be used to allocate nodes.
    ///
    /// # Parameters
    /// `metadata_region` : The start addr and size of the memory which will be used to store nodes.
    pub fn new(metadata_region: &Region) -> Self {
        // Initialize an allocator and set the head to none for now.
        HeapList {
            node_alloc: unsafe { HeapNodeAlloc::new(metadata_region) },
            head: None
        }
    }
    
    /// A method which adds a region to the linked list (heap). It also places it at the correct 
    /// position so the list of nodes is always sorted. Internally, it uses the HeapNodeAlloc to 
    /// allocate new nodes.
    ///
    /// # Parameters
    /// `region` : The new memory region which we want to allocate.
    /// `merge` : If we want to merge the region with previous ones if continous.
    ///
    /// # Returns
    /// Ok(new_node_ptr) if everything was successful, Err() if allocation failed.
    pub unsafe fn add(&mut self, region: &Region, merge: bool) -> Result<*mut HeapNode, ()> {
        // First allocate a new node, if not successful, return the Err.
        let mut new_node_ptr = self.node_alloc.alloc()?;
        
        // Assign the region to the node.
        (*new_node_ptr).region = region.clone();
        
        // Traverse the list from the head, to find the correct place for the node.
        let prev_node: Option<*mut HeapNode> = self.traverse(region.addr);
        
        // Check if the previous node was there (if we're at head or not).
        match prev_node {
            // If there was a previous node, just update it's next to the new node.
            Some(prev) => {
                (*new_node_ptr).next = (*prev).next;
                (*prev).next = Some(new_node_ptr);
            },
            
            // If there wasn't a previous node, set the new node as the head.
            None => {
                (*new_node_ptr).next = self.head;
                self.head = Some(new_node_ptr);
            },
        }
        
        // If merging was requested, merge all the possible nodes.
        if merge {
            self.merge();
        }
    
        // If we get here, the region was successfully added to a new node.
        Ok(new_node_ptr)
    }
    
    /// A method which removes a given node (with a pointer) from the list. It then returns the 
    /// region which was included in the node.
    ///
    /// # Parameters
    /// `node`: The pointer to the node we're deleting.
    ///
    /// # Returns 
    /// Ok(node_mem_region) if successful, Err() otherwise.
    pub unsafe fn remove(&mut self, node: *mut HeapNode) -> Result<Region, ()> {
        // Traverse the list from the head, to find where the given node is.
        let prev_node_ptr: Option<*mut HeapNode> = self.traverse((*node).region.addr);
        
        // To store the results of the removal.
        let mut to_return: Result<Region, ()> = Err(());
        
        // Check the value of the traversed node.
        match prev_node_ptr {
            // If there is currently a previous node, it means that we're not at the head.
            Some(prev_node) => {
                // In this case, check the next pointer.
                match (*prev_node).next {
                    // If there is a node, make sure it is the same as this node.
                    Some(curr_node) => {
                        // If it is, adjust the next pointers, and deallocate the node.
                        if curr_node == node {
                            (*prev_node).next = (*curr_node).next;
                            to_return = Ok((*curr_node).region.clone());
                            self.node_alloc.free(node)?;
                        }
                    },
                    
                    // Otherwise don't do anything (the error will remain).
                    None => (),
                }
            },
            
            // If there are not previous nodes, it means that we're at the head.
            None => {
                // Make sure the head is the same as our current node.
                if self.head == Some(node) {
                    // If it is, adjust the pointers, and deallocate the node.
                    to_return = Ok((*node).region);
                    self.head = (*node).next;
                    self.node_alloc.free(node)?;
                }
            }
        }
        
        to_return
    }
    
    /// A method which starts at the beginning of the list, and traverses it until it reaches a 
    /// node with and address (in it's region), right before the passeed address.
    ///
    /// # Parameters
    /// `addr` : The addr field of the node which we're traversing to.
    ///
    /// # Returns
    /// Some(prev_node_ptr) if that node exists, None otherwise.
    unsafe fn traverse(&self, addr: usize) -> Option<*mut HeapNode> {
        // To find which node pointer to return.
        let mut to_return: Option<*mut HeapNode> = None;
        
        // Traverse the list using an iterator.
        for node_ptr in self.into_iter() {
            // Check if the address of the current node is less than the passed address.
            if (*node_ptr).region.addr < addr {
                // If it is, "move forward".
                to_return = Some(node_ptr);
            } else {
                // If it's larger, just break.
                break
            }
        }
        
        // Return the found node pointer.
        to_return
    }
    
    /// A method which starts at the beginning, and iterates over the list and merges every two 
    /// cells that can be merged (based on their region). 
    pub unsafe fn merge(&mut self) {
        // To keep track of the last node (start at None since head's previous node is none).
        let mut prev_node: Option<*mut HeapNode> = None;
        
        // Go through every node in the list.
        for node in self.into_iter() {    
            // Check if we have a previous node.
            match prev_node {
                // If there is a value, check if it's continous with the current node.
                Some(prev) => {
                    // If it is continous, merge them.
                    if (*prev).region.end_addr() == (*node).region.addr {
                        // Adjust the size and next values.
                        (*prev).region.size += (*node).region.size;
                        (*prev).next = (*node).next;
                        
                        // Free the previous current node, and DO NOT update the prev_node.
                        self.node_alloc.free(node).expect("Couldn't free it!");
                    // If it's not continous, just update prev_node.
                    } else {
                        prev_node = Some(node);
                    }
                },
                
                // If not, just assign it the current node and update prev_node.
                None => prev_node = Some(node),
            }
        }
    }
    
    /// To get an iterator over HeapList. It simply stores the head in the iterator.
    pub fn into_iter(&self) -> HeapListIter {
        HeapListIter {
            curr: self.head.clone(),
        }
    }
    
    /// A function which iterates over this heap list, and dumps it to the console.
    pub fn dump(&self) {
        oxid_println!("Dumping the HeapList");
        // Get an iterator, dereferece the ptr, and dump it using debug mode.
        for node in self.into_iter() {
            oxid_println!("Node: {:?}", unsafe{*node});
        }
    }
}

/// An iterator type for the HeapList. It holds a current pointer.
pub struct HeapListIter {
    curr: Option<*mut HeapNode>,       // The current node in the list.
}


/// Implement the iterator trait for the HeapListIter so we can iterate over it.
impl Iterator for HeapListIter {
    type Item = *mut HeapNode;         // Type of item we're iterating over.
    
    /// To go through every element.
    fn next(&mut self) -> Option<Self::Item> {
        // Store the current item to return.
        let to_return = self.curr.clone();
        
        self.curr = match self.curr {
            Some(ptr) => unsafe { (*ptr).next },
            None => None,
        };
        
        // Return the stored pointer.
        to_return
    }
}

// Unit Tests **************************************************************************************

// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    use crate::mem::dyn_alloc::heap_node::HeapNode;
    use crate::mem::region::Region;

    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        unsafe {
            // Create a region at the end of the kernel.
            const SIZE: usize = core::mem::size_of::<HeapNode>() * 20;
            let start_addr = crate::mem::map::KERNEL_END_ADDR;
            let kreg = Region::new_sized(start_addr, SIZE);
        
            // Save the previously written data at address to restore it later.
            let prev_data: [u8; SIZE] = [0; SIZE];
            crate::olibc::memcpy::memcpy(prev_data.as_ptr() as *mut u8, start_addr as *const u8, SIZE);
            
            // Create a new heaplist.
            let mut list = super::HeapList::new(&kreg);
            
            // Create some test regions.
            let reg_1 = Region::new(1, 3);
            let reg_2 = Region::new(3, 11);
            let reg_3 = Region::new(12, 13);
            let reg_4 = Region::new(15, 110);
            
            // Add some values to it.
            let addr_1 = list.add(&reg_1, false).unwrap();
            list.add(&reg_2, false).unwrap();
            list.add(&reg_3, false).unwrap();
            
            // Remove a value, add another, and add the remove value again.
            list.remove(addr_1);
            list.add(&reg_4, true).unwrap();
            list.add(&reg_1, true).unwrap();   
            
            // Count the current nodes in the list.
            let mut count: usize = 0;
            for _node in list.into_iter() {
                count += 1;
            }
            
            // Check if the count is correct.
            assert_eq!(count, 3);
            
            // Restore the data from the backup.
            crate::olibc::memcpy::memcpy(start_addr as *mut u8, prev_data.as_ptr(), SIZE);
        }
    }
}
