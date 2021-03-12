//! A struct which represents the basic memory information in the multiboot information.
//! It's definition is directly derived from the multiboot2 specifications, which can be found at 
//! https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
//!
//! Author: Ardalan Ahanchi
//! Date: Jan 2021

/// A structure which allows the kernel to see the size of the lower and upper memory regions.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MemInfo {
    tag_type: u32,              // Type of the tag (a number between 0 to 10).
    tag_size: u32,              // The size of the tag in bytes.
    pub mem_lower: u32,         // Amount of the lower memory (in kilobytes), and starts at 0.
    pub mem_upper: u32,         // Amount of the higher memory (in kilobytes), and starts at 1MB.
}

impl MemInfo {
    /// The default constructor which parses the information restored at the given address, and 
    /// initializes a new MemInfo struct and returns it.
    ///
    /// # Parameters
    /// `addr` : The address where this tag starts (it should be 8 byte aligned).
    ///
    /// # Returns
    /// The parsed memory information struct.
    pub unsafe fn new(addr: usize) -> Self {
        // Since in this case the structure is very simple, just dereference it (every single 
        // value in this structure is static as far as it's size goes).
        // In this case, it returns a copy of it.
        *(addr as *const MemInfo)
    }
}
