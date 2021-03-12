//! A file that represents the x86_64 page tabe. It is the fourth level in
//! the 4-level paging schema. The actual addresses of the frames are stored here.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]                        // To allow partial usage.

use crate::mem::bitwise::BitWise;           // To allow bitwise operations.

/// Define an entry for PT and implement all the general functions for it using the defined macro
/// in general_entry.
/// Based on the AMD64 manual volume 2 in pages 135 and 144, this is following entry structure.
/// Additionally, https://os.phil-opp.com/page-tables/ was helpful.
/// [0] - P - Present - 0 is an unused entry, 1 is used.
/// [1] - R/W - Read/Write - 0 is read-only page, 1 is read/write.
/// [2] - U/S - User/Super - 0 is kernel mode, 1 is user programs.
/// [3] - PWT - Page-level wirethrough - 0 uses a Writeback caching policy, 1 uses a Writethrough.
/// [4] - PCD - Page-level cache disable - 0 makes the table cacheable, 1 is not.
/// [5] - A - Accessed - 1 if the page was used.
/// [6] - D - Dirty - 1 if the physical page was written to. It is set by the CPU.
/// [7] - PAT - Page attribute table - High order bit of a 3-bit index into the PAT register.
/// [8] - G - Global page - 1 if the frame is a global page, 0 otherwise.
/// [9,11] - AVL - Available to use.
/// [12,51] - ADR - Address - The physical address of the frame (the bits 0-11 are 0).
/// [52,58] - AVL - Available to use.
/// [59,62] - Depends - Usage is based on value in CR4.PKE. 
/// [63] - NX - No execute - If set, no code can be executed here.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PTEntry(usize);
impl_general_entry!(PTEntry);

/// Implement additional functions which are applicable to this type of entry specifically.
impl PTEntry {
    /// A method which determines if the current page was written to (set by the CPU).
    ///
    /// # Returns
    /// true if the page was written to, false otherwise.
    pub fn is_dirty(&self) -> bool {
        self.0.is_set(6)
    }
    
    /// A method which determines if the current page is a global page.
    ///
    /// # Returns
    /// true if the page is global, false otherwise.
    pub fn is_global(&self) -> bool {
        self.0.is_set(8)
    }
    
    /// A method which sets the global bit and makes this page global.
    ///
    /// # Parameters
    /// `is_global` : True if it is global, false otherwise.
    pub fn set_global(&mut self, is_global: bool) {
        self.0.write_bit(8, is_global)
    }
    
    
}

/// Define the PT with a virtual address and a certain number of entries.
pub struct PT {
    virt_addr: usize,
    num_entries: usize,
}

impl_general_table!(PT, PTEntry);

impl PT {
    /// A method which accepts a virtual address, and returns the index within PT for that addr.
    /// this is based on the architecture definitions which is defined in the AMD64 manual volume 2
    /// 136. This specifically uses bits 12 to 20 (inclusive).
    ///
    /// # Parameters
    /// `virt_addr` : The virtual address which we're trying to map.
    ///
    /// # Returns
    /// The index within the PT for that specific address.
    pub fn get_idx(virt_addr: usize) -> usize {   
        // Shift right to start at bit 12, then bitmask it to only get the first 9 bits.
        (virt_addr >> 12) & 0x1FF
    }
}
