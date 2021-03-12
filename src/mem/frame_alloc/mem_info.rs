//! A sub-module which can calculate the memory information based on the passed multiboot 
//! information structures. It finds where the usable and unsable kernel areas are.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021


use crate::multiboot2::MultibootInfo; // To find out where to put the bit field, and memory size.
use crate::mem::region::Region;       // To allow using memory regions.

/// A function which calculates where the kernel ends. It uses the parsed elf symbols table in the 
/// multiboot2 information header. Additionally, it checks the address of multiboot2 header and 
/// takes that into consideration (includes it as the "kernel").
///
/// # Parameters
/// `mb_info` : The multiboot information reference (already parsed).
///
/// # Returns
/// The address where the kernel ends and where we can start allocating frames from.
pub fn get_kernel_end(mb_info: &MultibootInfo) -> usize {
    // Define a value for the kernel end and start it as 0.
    let mut curr_kernel_end: usize  = 0;

    // Go through elf symbols, find the last entry and set it as kernel end for now.
    let elf_syms = mb_info.elf_symbols_tag.unwrap();
    for sym in elf_syms {
        // If the address + size is larger than the current latest address, update it.
        let curr_symbol_end = sym.sh_addr + sym.sh_size as usize;
        if  curr_symbol_end > curr_kernel_end {
            curr_kernel_end = curr_symbol_end;
        }
    }
    
    // Check if the mb_info is later than the kernel_end, if it is, update it.
    let mb_end = mb_info.ptr + mb_info.total_size;
    if mb_end > curr_kernel_end {
        curr_kernel_end = mb_end;
    }
    
    // Now we found where we can start mapping from.
    curr_kernel_end
}

/// A function which finds the size of the memory from the multiboot info structure. It basically 
/// returns the maximum address of this system.
///
/// # Parameters
/// `mb_info` : The multiboot information reference (already parsed).
///
/// # Returns
/// The size of memory which is the last addressable physical address.
pub fn get_mem_end(mb_info: &MultibootInfo) -> usize {
    // Define a value for the address of the end of memory.
    let mut curr_mem_end: usize = 0;

    // Go through the multiboot memory map, and check the largest memory end.
    let mem_map = mb_info.mem_map_tag.unwrap();
    for map in mem_map {
        // If the base address + length is larger than the current end, update it.
        let curr_map_end = (map.base_addr + map.length) as usize;
        if curr_map_end > curr_mem_end {
            curr_mem_end = curr_map_end;
        }
    }
    
    // Now we found where we can start map to.
    curr_mem_end
}

/// A function which creates a memory region representing the end of the kernel to the end of the 
/// memory. It does not do any alignment by itself and uses the data passed to it.
///
/// # Parameters
/// `mb_info` : The multiboot information reference (already parsed).
///
/// # Returns
/// A region which represents the usable physical memory after the kernel.
pub fn get_usable_region(mb_info: &MultibootInfo) -> Region {
    // Simply calculate the region and return it.
    Region::new(get_kernel_end(mb_info), get_mem_end(mb_info))
}

/// A function which creates a memory region representing the adderss used by the kernel and it's 
/// related structrued. It does not do any alignment by itself and uses the data passed to it.
///
/// # Parameters
/// `mb_info` : The multiboot information reference (already parsed).
///
/// # Returns
/// A region which starts at address 0, and ends where the kernel ends.
pub fn get_kernel_region(mb_info: &MultibootInfo) -> Region {
    // Simply calculate the region and return it.
    Region::new(0, get_kernel_end(mb_info))
}
