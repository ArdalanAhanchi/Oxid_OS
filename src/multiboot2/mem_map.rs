//! A struct which represents the memory map tag in the multiboot info structure.
//! It's definition is directly derived from the multiboot2 specifications, which can be found at 
//! https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
//!
//! Author: Ardalan Ahanchi
//! Date: Jan 2021

#![allow(dead_code)]

/// A strcture which represents memory map as it is used by the outside programs. It is used 
/// to provide an interface to the memory map sections.
#[derive(Copy, Clone)]
pub struct MemMap {
    num_entries: usize,
    curr_entry_idx: usize,
    entries_addr: usize,
}

/// The memory accurate representation of the memory map type, it is as defined in the multiboot2
/// specifications. It will be followed with entries for the memory map.
#[repr(C, packed)]
struct MemMapRepr {
    tag_type: u32,              // Type of the tag.
    tag_size: u32,              // The size of the tag in bytes.
    pub ent_size: u32,          // The size for each entry.
    pub ent_ver: u32,           // Currently 0, used for future revisions.
}

impl MemMap {
    /// The default constructor which parses the information restored at the given address, and 
    /// initializes a new MemInfo struct and returns it.
    ///
    /// # Parameters
    /// `addr` : The address where this tag starts (it should be 8 byte aligned).
    /// `size` : The total size of the tag (including the headers).
    ///
    /// # Returns
    /// The parsed memory information struct.
    pub unsafe fn new(addr: usize, size: usize) -> Self {
        // Parse the raw information into the struct.
        let mem_map = &*(addr as *const MemMapRepr);

        // Create a new memmap struct, start the idx at 0, calculate the number of entries and save 
        // it, and then calculate the starting address of the entries.    
        MemMap {
            num_entries: (size - core::mem::size_of::<MemMapRepr>()) / mem_map.ent_size as usize,
            curr_entry_idx: 0,
            entries_addr: addr + core::mem::size_of::<MemMapRepr>(),
        }
    }
}

// Implement the iterator trait for the entires, so we can go over them using a simple for loop.
impl Iterator for MemMap {
    /// Define the type of the item used in the iterator (in this case it's the memory map entries).
    type Item = MemMapEnt;
    
    /// A function which proceeds to the next value in the iterator. 
    ///
    /// # Returns
    /// A Some(MemMapEnt) if the next is in range, None otherwise.
    fn next(&mut self) -> Option<MemMapEnt> {
        // Check if the current index is in range.
        if self.curr_entry_idx < self.num_entries as usize {
            // Calculate the memory location, cast it, and read it.
            let curr_ent = unsafe { &*((self.entries_addr
                + (self.curr_entry_idx * core::mem::size_of::<MemMapEnt>())) as *const MemMapEnt) };
            
            // Go to the next entry.
            self.curr_entry_idx += 1;
            
            // Return a copy of the entry which was read.
            Some(*curr_ent)
        } else {
            None
        }
    }
}

/// Represents each entry for the memory map entries (which will follow the MemMapRepr). It will 
/// be used for the iterator, and it's definition is based on the multiboot 2 standard.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MemMapEnt {
    pub base_addr: u64,         // Starting physical address.
    pub length: u64,            // Size of memory region in bytes.
    pub ent_type: u32,          // Variety of region. As defined by MemMapEntType
    reserved: u32,         // Address where the first byte of the section is at.
}

/// Each type for the memory map entries. This corresponds to the ent_type in the struct MemMapEnt.
/// it's values are defined by the multiboot 2 standard.
#[repr(C)]
pub enum MemMapEntType {
    Available = 1,              // Ram which is ready to use.
    AcpiInfo = 3,               // Usable memory holding ACPI information.
    ReservedMem = 4,            // Reserved memory which has to be preserved for hibernation.
    Defective = 5,              // Memory which is occupied by defective RAM.
}
