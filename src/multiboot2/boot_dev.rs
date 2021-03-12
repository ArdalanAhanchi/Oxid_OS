//! A struct which represents which bios disk device boot loader loaded the OS image from.
//! It's definition is directly derived from the multiboot2 specifications, which can be
//! found at https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
//!
//! Author: Ardalan Ahanchi
//! Date: Jan 2021

/// The base structure which indicates which BIOS boot device the bootloader found the OS on.
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct BootDev {
    tag_type: u32,              // Type of the tag (a number between 0 to 10).
    tag_size: u32,              // The size of the tag in bytes.
    pub bios_dev: u32,          // The bios device number understood by INT 0x13.
    pub partition: u32,         // Top level partition number.
    pub sub_partition: u32,     // the BSD sub-partition within the DOS parition.
}

impl BootDev {
    /// The default constructor which parses the information restored at the given address, and 
    /// initializes a new BootDev struct and returns it.
    ///
    /// # Parameters
    /// `addr` : The address where this tag starts (it should be 8 byte aligned).
    ///
    /// # Returns
    /// The parsed boot device information struct.
    pub unsafe fn new(addr: usize) -> Self {
        // Since in this case the structure is very simple, just dereference it (every single 
        // value in this structure is static as far as it's size goes).
        // In this case, it returns a copy of it.
        *(addr as *const BootDev)
    }
}
