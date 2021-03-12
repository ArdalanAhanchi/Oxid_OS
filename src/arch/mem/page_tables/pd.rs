//! A file that represents the x86_64 page directory. It is the third level in
//! the 4-level paging schema. The page tables will be accessed from here.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]                        // To allow partial usage.

use super::PT;

/// Define an entry for PD and implement all the general functions for it using the defined macro
/// in general_entry.
/// Based on the AMD64 manual volume 2 in pages 135 and 142, this is following entry structure.
/// Additionally, https://os.phil-opp.com/page-tables/ was helpful.
/// [0] - P - Present - 0 is an unused entry, 1 is used.
/// [1] - R/W - Read/Write - 0 is read-only page, 1 is read/write.
/// [2] - U/S - User/Super - 0 is kernel mode, 1 is user programs.
/// [3] - PWT - Page-level wirethrough - 0 uses a Writeback caching policy, 1 uses a Writethrough.
/// [4] - PCD - Page-level cache disable - 0 makes the table cacheable, 1 is not.
/// [5] - A - Accessed - 1 if the page was used.
/// [6] - IGN - Ignored.
/// [7] - 0 - Always 0.
/// [8] - IGN - Ignored.
/// [9,11] - AVL - Available to use.
/// [12,51] - ADR - Address - The physical address of the frame (the bits 0-11 are 0).
/// [52,62] - AVL - Available to use.
/// [63] - NX - No execute - If set, no code can be executed here.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PDEntry(usize);
impl_general_entry!(PDEntry);
impl_make_table_if_not_present!(PDEntry, PT);

/// Define the PD table with a virtual address and a certain number of entries.
pub struct PD {
    virt_addr: usize,
    num_entries: usize,
}

impl_general_table!(PD, PDEntry);

impl PD {
    /// A method which accepts a virtual address, and returns the index within PD for that addr.
    /// this is based on the architecture definitions which is defined in the AMD64 manual volume 2
    /// 136. This specifically uses bits 21 to 29 (inclusive).
    ///
    /// # Parameters
    /// `virt_addr` : The virtual address which we're trying to map.
    ///
    /// # Returns
    /// The index within the PD for that specific address.
    pub fn get_idx(virt_addr: usize) -> usize {   
        // Shift right to start at bit 21, then bitmask it to only get the first 9 bits.
        (virt_addr >> 21) & 0x1FF
    }
}
