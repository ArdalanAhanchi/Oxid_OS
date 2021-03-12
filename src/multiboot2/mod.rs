//! A submodule which handles the parsing of the multiboot_info structure which is passed to 
//! the kernel by any multiboot complient boot loader.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

#![allow(dead_code)]

mod tag;
mod mem_info;
mod boot_dev;
mod elf_symbols;
mod mem_map;

#[allow(unused_imports)]
use tag::{Tag, TagType};

/*
pub mod boot_cmd;
pub mod modules;
pub mod boot_loader;
pub mod apm_table;
pub mod vbe_info;
pub mod framebuffer_info;
pub mod termination;
*/

/// The base structure which is at the beginning of the boot information section.
#[repr(C, packed)]
pub struct BaseInfo {
    pub total_size: u32,        // Total size of boot information in bytes.
    reserved: u32,              // Always should be zero and ignored.
}

/// A structure which represents the parsed return type. This is only implemented partially to 
/// only utilize what the kernel needs for basic operation. In other words, it does not include 
/// all the tags which are present.
pub struct MultibootInfo {
    pub ptr: usize,                                       // Where the structure is located at.
    pub total_size: usize,                                // The total size of the header. 
    pub mem_info_tag: Option<mem_info::MemInfo>,          // The rest of the tags.
    pub boot_dev_tag: Option<boot_dev::BootDev>,
    pub elf_symbols_tag: Option<elf_symbols::ElfSymbols>,
    pub mem_map_tag: Option<mem_map::MemMap>,
}

impl MultibootInfo {
    // A function which parses the multiboot info passed. It uses the address passed and 
    // tries to parse the supported types. It will then return the results.
    ///
    /// # Parameters
    /// `multiboot_info` : The address for the multiboot info structure.
    ///
    /// # Returns
    /// A new instance of Multiboot info if it could parse it, otherwise returns an Err. 
    pub unsafe fn parse(multiboot_info: usize) -> Result<MultibootInfo, ()> {
        // Get the base information (specifically total_size).
        let base_info = &*(multiboot_info as *const BaseInfo);
        
        // If the multiboot information is not valid, return.
        if ! info_is_valid(multiboot_info, base_info.total_size) {
            oxid_err!("Invalid multiboot2 information structure.");
            return Err(());
        }
        
        // Create the structure which will be returned (which holds the information).
        let mut parsed_info = MultibootInfo {
                ptr: multiboot_info,
                total_size: base_info.total_size as usize,
                mem_info_tag: None,
                boot_dev_tag: None,
                elf_symbols_tag: None,
                mem_map_tag: None,
        };
        
        // Store the current pointer for parsing.
        let mut curr_ptr: usize = multiboot_info + core::mem::size_of::<BaseInfo>();
        let end_ptr: usize = curr_ptr + (*base_info).total_size as usize;
        
        // Go through every single structure.
        while curr_ptr < end_ptr {
            // Read the current tag and cast it to the tag type.
            let curr_tag = &*(curr_ptr as *const Tag);
            
            // Parse the tag and store in parsed_info.
            parse_tag(&mut parsed_info, curr_ptr, &curr_tag);
        
            // Increase the current pointer by the number of bytes which the current tag used.
            curr_ptr += curr_tag.tag_size as usize;
            
            // If the increased pointer is not aligned, align it.
            if curr_ptr % tag::TAG_ALIGNMENT != 0 {
                curr_ptr += tag::TAG_ALIGNMENT - curr_ptr % tag::TAG_ALIGNMENT;
            }
            
            // If we've reached the termination tag, exit.
            if curr_tag.tag_type == tag::TagType::Termination as u32 
                && curr_tag.tag_size == tag::TERMINATION_TAG_SIZE {
                break;
            }
        }
        
        Ok(parsed_info)
    }
}


/// A function which validates the multiboot info structure by checking it's end tag. It checks the 
/// size and the type of the end tag to ensure everything is as expected.
///
/// # Parameters
/// `multiboot_info` : The address of the multiboot info structure.
/// `total_size` : The size which is read from the base_info at the beginning of multiboot_info.
///
/// # Returns
/// True if the structure is valid, false otherwise.
unsafe fn info_is_valid(multiboot_info: usize, total_size: u32) -> bool {
    // Get the actual termination tag by calculating it's offset from beginning (using total_size).
    let termination_tag = &*((multiboot_info + (total_size - tag::TERMINATION_TAG_SIZE) 
        as usize) as *const Tag);

    // Check if the type and size are correct.
    termination_tag.tag_type == tag::TagType::Termination as u32
         && termination_tag.tag_size == tag::TERMINATION_TAG_SIZE
}

/// A function which checks the type of the current tag, and sets the corresponding structure for it
/// in the given MultibootInfo struct. It is used to call the correct constructors based on the 
/// type passed to it.
///
/// # Parameters
/// `info` : The output structure where the new tag will be set (which will be returned).
/// `addr` : The starting address for the tag (which includes the size and type).
/// `tag_h`: The parsed tag header which includes the type and the size.
unsafe fn parse_tag(info: &mut MultibootInfo, addr: usize, tag_h: &tag::Tag) {
    // Convert the given tag number to the type enum.
    let tag_type_enum = tag::TagType::from(tag_h.tag_type);

    // Based on the type of the tag, call their constructors.
    match tag_type_enum {
        tag::TagType::MemInfo => { info.mem_info_tag = Some(mem_info::MemInfo::new(addr)); },
        tag::TagType::BootDev => { info.boot_dev_tag = Some(boot_dev::BootDev::new(addr)); },
        tag::TagType::ElfSymbols => { info.elf_symbols_tag = Some(elf_symbols::ElfSymbols::new(addr)); },
        tag::TagType::MemMap => { info.mem_map_tag = Some(mem_map::MemMap::new(addr, tag_h.tag_size as usize)); },
        _ => {}
    }
}
