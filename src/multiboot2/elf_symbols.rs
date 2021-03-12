//! A struct which represents the elf symbols tag in the multiboot info structure.
//! It's definition is directly derived from the multiboot2 specifications, which can be found at 
//! https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html
//!
//! Author: Ardalan Ahanchi
//! Date: Jan 2021

#![allow(dead_code)]

/// A strcture which represents the memory information as viewed by outside programs. It is used 
/// to provide an interface to the elf section headers.
#[derive(Copy, Clone)]
pub struct ElfSymbols {
    pub num: u32,                       // The total number of section headers.
    pub ent_size: u32,                  // The size for each entry.
    pub shndx: u32,                     // Relevant section header table index.
    curr_header_idx: usize,             // The current index for the iterator.
    section_header_addr: usize,         // The address of the first section header.
}

/// The base structure which contains the section header table from the kernel itself. This is the 
/// raw representation as it will apear in the memory. This table seems to be incorrectly defined 
/// in the official mb2 specifications (for 64 bits in general). More information about these 
/// issue is explained in: https://github.com/rust-osdev/multiboot2-elf64/blob/master/README.md
#[repr(C, packed)]
struct Elf64SymbolsRepr {
    tag_type: u32,              // Type of the tag.
    tag_size: u32,              // The size of the tag in bytes.
    pub num: u32,               // The number of section headers.
    pub ent_size: u32,          // The size for each entry.
    pub shndx: u32,             // Relevant section header table index.
}

impl ElfSymbols {
    /// The default constructor which parses the information restored at the given address, and 
    /// initializes a new elf symbols struct and returns it.
    ///
    /// # Parameters
    /// `addr` : The address where this tag starts (it should be 8 byte aligned).
    ///
    /// # Returns
    /// The parsed elf symbols struct.
    pub unsafe fn new(addr: usize) -> Self {
        // Parse the raw information into the struct.
        let info = &*(addr as *const Elf64SymbolsRepr);

        // Create a new struct which the same values as info, but also set the idx to 0 (since we're
        // starting the count), and calculate the address for the section header.    
        ElfSymbols {
            num: info.num,
            ent_size: info.ent_size,
            shndx: info.shndx,
            curr_header_idx: 0,
            section_header_addr: addr + core::mem::size_of::<Elf64SymbolsRepr>(),
        }
    }
}

// Implement the iterator trait for the symbols, so we can go over them using a simple for loop.
impl Iterator for ElfSymbols {
    /// Define the type of the item used in the iterator (in this case it's the section headers).
    type Item = Shdr;
    
    /// A function which proceeds to the next value in the iterator. 
    ///
    /// # Returns
    /// A Some(Shdr) if the next is in range, None otherwise.
    fn next(&mut self) -> Option<Shdr> {
        // Check if the current index is in range.
        if self.curr_header_idx < self.num as usize {
            // Calculate the memory location, cast it, and read it.
            let curr_shdr = unsafe { &*((self.section_header_addr 
                + (self.curr_header_idx * core::mem::size_of::<Shdr>())) as *const Shdr) };
            
            // Go to the next header.
            self.curr_header_idx += 1;
            
            // Return a copy of the Shdr which was read.
            Some(*curr_shdr)
        } else {
            None
        }
    }
}



/// The section header structure which is defined in the ELF specifications. More information can 
/// be found at: https://man7.org/linux/man-pages/man5/elf.5.html . 
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Shdr {
    pub sh_name: u32,           // Name of the section (index into the section header string table).
    pub sh_type: u32,           // Categories of the contents.
    pub sh_flags: u64,          // Description of misc. attibutes (by using single bit flags).
    pub sh_addr: usize,         // Address where the first byte of the section is at.
    pub sh_offset: usize,       // Byte offset from the begining of the file to first byte.
    pub sh_size: u64,           // Section's size in bytes.
    pub sh_link: u32,           // Header table index link.
    pub sh_info: u32,           // Extra information (depending on the type).
    pub sh_addralign: u64,      // Address alignment constraints. 
    pub sh_entsize: u64,        // Size for each entry if it includes this table.
}

/*
// TODO: Move this to the loader code (since it will utilize ELF).

/// The types defined for the Shdr (correspond to sh_type). These are defined completely in:
/// http://refspecs.linux-foundation.org/LSB_2.1.0/LSB-Core-generic/LSB-Core-generic/elftypes.html
#[repr(C)]
pub enum ShdrType {
    Null = 0,               // Inactive section header.
    ProgBits = 1,           // Information defined by the program.
    SymTab = 2,             // Holds a symbol table.
    StrTab = 3,             // Holds a string table.
    Rela = 4,               // Holds relocation entries (with addends).
    Hash = 5,               // Holds a symbol hash table.
    Dynamic = 6,            // Holds information for dynamic linking.
    Note = 7,               // Holds information that marks the file.
    NoBits = 8,             // Uses no space in the file.
    Rel = 9,                // Holds relocation entries (without addends).
    ShLib = 10,             // Reserved (The semantics are not specified).
    DynSym = 11,            // Holds a set of symbols which are used for dynamic linking.
    LoProc = 0x70000000,    // Values reserved for processor specific symantics.
    HiProc = 0x7fffffff,    // Values reserved for processor specific symantics. 
    LoUser = 0x80000000,    // Values reserved for user programs (lower bound).
    HiUser = 0xffffffff,    // Values reserved for user programs (higher bound).
}
*/
