//! A sub-module which represents a node in the linked list of memory regions managed by the 
//! allocator. Each node includes a region, and some other fields for managing linked lists.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

use crate::mem::region::Region;                    // To represent memory regions.

/// A structure which represents a node for managing memory.
#[derive(Copy, Clone, Debug, Default)]
pub struct HeapNode {
    pub region: Region,                            // The region that this node represents.
    pub next: Option<*mut HeapNode>,               // Pointer to the next node.
    pub list_idx: usize,                           // Store index to allow fast frees.
}
