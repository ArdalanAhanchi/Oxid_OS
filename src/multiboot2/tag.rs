//! A struct which represents the general tag (which could be of many types). This is used to 
//! check the tag type and size easily. It's definition is directly derived from the multiboot2
//! specifications, which can be found at 
//! https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
//!
//! Author: Ardalan Ahanchi
//! Date: Jan 2021

pub const TERMINATION_TAG_SIZE: u32 = 8;            // Correct size for the termination tag.            
pub const TAG_ALIGNMENT: usize = 8;                 // The alignment for each tag.

/// The base structure which is at the beginning of the boot information section.
#[repr(C, packed)]
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Tag {
    pub tag_type: u32,      // The type (which are defined at TagType).
    pub tag_size: u32,      // The size of the tag (general data structure size).
}

/// An enum which represents each tag type with it's corresponding value. This is used to understand
/// which tag is currently pointed to by the memory.
#[repr(u32)]
#[allow(dead_code)]
pub enum TagType {
    Termination = 0,
    BootCmd = 1,
    BootLoader = 2,
    Modules = 3,
    MemInfo = 4,
    BootDev = 5,
    MemMap = 6,
    VbeInfo = 7,
    FramebufferInfo = 8,
    ElfSymbols = 9,
    ApmTable = 10,
    Efi32SysTablePtr = 11,
    Efi64SysTablePtr = 12,
    SMBiosTables = 13,
    OldRSDP = 14,
    NewRSDP = 15,
    NetInfo = 16,
    EFIMemMap = 17,
    EFIBootServicesNT = 18,
    EFI32ImgHandlePtr = 19,
    EFI64ImgHandlePtr = 20,
    ImgLoadBasePhysAddr = 21,
    Invalid,
}

/// Allow the conversion of u32 to TagTypes.
impl From<u32> for TagType {

    /// Allow the conversion of a u32 to tag type. If the given number is not within the defined
    /// values, it will return an invalid type.
    ///
    /// # Parameters
    /// `val` : The integer which we want to convert.
    ///
    /// # Returns
    /// The corresponding tag type which we can use in a match statement.
    fn from(val: u32) -> Self {
        // Match it to the correct tag type.
        match val {
            0 => TagType::Termination,
            1 => TagType::BootCmd,
            2 => TagType::BootLoader,
            3 => TagType::Modules,
            4 => TagType::MemInfo,
            5 => TagType::BootDev,
            6 => TagType::MemMap,
            7 => TagType::VbeInfo,
            8 => TagType::FramebufferInfo,
            9 => TagType::ElfSymbols,
            10 => TagType::ApmTable,
            11 => TagType::Efi32SysTablePtr,
            12 => TagType::Efi64SysTablePtr,
            13 => TagType::SMBiosTables,
            14 => TagType::OldRSDP,
            15 => TagType::NewRSDP,
            16 => TagType::NetInfo,
            17 => TagType::EFIMemMap,
            18 => TagType::EFIBootServicesNT,
            19 => TagType::EFI32ImgHandlePtr,
            20 => TagType::EFI64ImgHandlePtr,
            21 => TagType::ImgLoadBasePhysAddr,
            _ => TagType::Invalid,                  // If any other type, return invalid.
        }
    }
}
