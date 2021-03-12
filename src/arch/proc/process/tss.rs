//! A sub-module which represents a tasks state segment as defined by the architecture.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

#![allow(dead_code)]

/// The gdt entry for TSS. It sets the type to TSS, and makes it present (more info in AMD manuals).
const TSS_GDT_ENT: usize = (1 << 40) | (1 << 43) | (1 << 47);

/// The maximum number of TSS entries (defined in the arch/boot/boot.asm).
const GDT_MAX_TSS_ENTRIES: u16 = 1;

/// A structure which represents the x86_64 task state segment. It is used for managing processes.
/// More details can be found at: https://wiki.osdev.org/Task_State_Segment and 
/// https://github.com/grahamedgecombe/arc/blob/master/kernel/arc/cpu/tss.h and
/// https://os.phil-opp.com/double-faults/ and 
/// https://en.wikipedia.org/wiki/Task_state_segment and
/// https://forum.osdev.org/viewtopic.php?t=13678
#[derive(Copy, Clone, Default)]
#[repr(C, packed)]
pub struct TSS {
    reserved_0: u32,
    pub rsp_0: usize,               // The stack pointers 0-2 (used when changing "rings").
    pub rsp_1: usize,
    pub rsp_2: usize,
    reserved_1: usize,
    pub ist_1: usize,               // Interrupt stack table pointers 1-7.
    pub ist_2: usize, 
    pub ist_3: usize,
    pub ist_4: usize, 
    pub ist_5: usize,
    pub ist_6: usize, 
    pub ist_7: usize,
    reserved_2: usize,
    reserved_3: usize,
    pub iopb_offset: usize,         // For IO bitmap (for here it will be the size of TSS).
}

/// Wrappers for the assembly functions.
extern "sysv64" {
    /// A function which returns the adderss of the gdt table set up by arch/boot/boot.asm. This is 
    /// the main (and only) gdt table for this kernel.
    ///
    /// # Returns
    /// The address of the global descriptor table.
    fn get_gdt_addr() -> usize;
    
    /// A function which allows the code to obtain where the 0th TSS entry starts in the GDT.
    ///
    /// # Returns
    /// the offset (num bytes) of the 0th TSS entry from the GDT base addr.
    fn get_gdt_tss_offset() -> u16;
    
    /// A wrapper for the LTR instruction which loads a TSS at a given offset in the GDT.
    ///
    /// # Parameters
    /// `offset` : The offset to the specific TSS entry in the GDT.
    fn load_task_register(offset: u16);
}

impl TSS {
    /// A constant constructor which simply initializes everything and sets all to 0. This is used
    /// to create an empty task state segment.
    ///
    /// # Returns
    /// An empty TSS with everything set to zeroes (except iopb_offset).
    pub const fn new() -> Self {
        // Create an entry with everything set to zeros and save the size in iopb offset.
        TSS {
            reserved_0: 0,
            rsp_0: 0,
            rsp_1: 0,
            rsp_2: 0,
            reserved_1: 0,
            ist_1: 0,
            ist_2: 0, 
            ist_3: 0,
            ist_4: 0, 
            ist_5: 0,
            ist_6: 0, 
            ist_7: 0,
            reserved_2: 0,
            reserved_3: 0,
            iopb_offset: core::mem::size_of::<TSS>(),
        }
    }
    
    /// A method which loads the current TSS into the CPU. It first sets the address of itself in 
    /// the corresponding GDT entry, and then calls the ltr instruction to load itself.
    ///
    /// # Parameters
    /// `gdt_tss_idx` : The index within the TSS entries in the gdt (0 by default).
    pub unsafe fn load(&self, gdt_tss_idx: u16) {
        // Make sure the passed index is valid.
        assert!(gdt_tss_idx < GDT_MAX_TSS_ENTRIES);
    
        // Get the offset of the current tss from the assembly code.
        let gdt_tss_offset = get_gdt_tss_offset() 
            + (gdt_tss_idx * (core::mem::size_of::<usize>() * 2) as u16);
            
        // Also get the address of gdt and calculate the current gdt address.
        let gdt_tss_addr = get_gdt_addr() + (gdt_tss_offset as usize);
        
        // Get pointers to low and high addresses.
        let tss_low_ptr = gdt_tss_addr as *mut usize;
        let tss_high_ptr = (gdt_tss_addr + core::mem::size_of::<usize>()) as *mut usize;
        
        // Get the address of this tss.
        let tss_addr = self as *const TSS as usize;

        // Clear the entries for now.        
        *tss_low_ptr = TSS_GDT_ENT;
        *tss_high_ptr = 0;
        
        // Set the base address of tss based on the architecture specifications.
        // this is the same as any 16-bit gdt entry.
        *tss_low_ptr |= (core::mem::size_of::<TSS>() - 1) & 0xFFFF;
        *tss_low_ptr |= (tss_addr & 0xFFFFFF) << 16;
        *tss_low_ptr |= ((tss_addr >> 24) & 0xFF) << 56;
        *tss_high_ptr |= (tss_addr >> 32) & 0xFFFFFFFF;
        
        // Actually load the register using the offset and the ltr instruction.
        load_task_register(gdt_tss_offset);
    }
}
