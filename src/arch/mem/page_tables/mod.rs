//! A sub-module which handles the intel 4-level paging. It implements the page_table
//! trait which allows it to map and unmap pages and handle all the levels needed for
//! the paging (all the structures).
//! 
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021.

#![allow(dead_code)]

#[macro_use]
mod general_entry;  // Represent an entry which is used by all the levels.

#[macro_use]
mod general_table;  // Represent an table which is used by all the levels.

// Each of these modules represents one level of the page tables. More detailed explanation can be
// found in intel or AMD manuals. Additionally, here is a great explanation:
// https://www.iaik.tugraz.at/teaching/materials/os/tutorials/paging-on-intel-x86-64/
mod pml_4;          // Page map level-4
mod pdp;            // Page directory pointer
mod pd;             // Page directory
mod pt;             // Page table

use pml_4::PML4;
use pdp::PDP;
use pd::PD;
use pt::PT;

// Calculate the total number of entries for all tables (by default it is 512).
pub const NUM_ENTRIES: usize = crate::mem::frame_alloc::FRAME_SIZE / core::mem::size_of::<usize>();  

// Calculate the start and end of virtual memory region for the page tables (based on self ent).
pub const PAGE_TABLES_VM_START: usize = PT_START_ADDR;
pub const PAGE_TABLES_VM_END: usize = PML4_START_ADDR | 0xFFFF;

// The index for the self-ref entry (page tables addresses).
const SELF_ENTRY_IDX: usize = 511;          

// Calculate the sign extension bits (canonical) based on the given self_entry index.
const SIGN_EXTEND: usize = match SELF_ENTRY_IDX > (NUM_ENTRIES / 2) {
    true => 0xFFFF << 48,
    false => 0,
};

// Calculate the starting address of each table (using the self-referenced entries).
const PT_START_ADDR: usize = SIGN_EXTEND | (SELF_ENTRY_IDX << 39);      // Start addr of PTs.
const PD_START_ADDR: usize = PT_START_ADDR | (SELF_ENTRY_IDX << 30);    // Start addr of PDs.
const PDP_START_ADDR: usize = PD_START_ADDR | (SELF_ENTRY_IDX << 21);   // Start addr of PDPs.
const PML4_START_ADDR: usize = PDP_START_ADDR | (SELF_ENTRY_IDX << 12); // Start addr of PML4.

/// Create a public struct which represents the 4 level paging from the outside world. It implements
/// the page table trait which allows it to be more modular and makes it possible to utilize other 
/// paging techniques in the future.
#[repr(C)]
pub struct PageTables {
    table_addr: Option<usize>,  // The address of the PML4.
}

impl PageTables {

    /// A default constructor which simply creates an empty PML4 and returns a reference to this
    /// page table.
    pub const fn new() -> Self {
        // Set the adderss to 0 until the page tables are set-up, and make a mutex and return it.
        PageTables {
            table_addr: None,
        }
    }
    
    /// A method which sets up the kernel page table. It gets the address of the current page table,
    /// sets the table address, sets the self-reference entry, 
    pub fn setup_kernel_pagetable(&mut self) {
        unsafe {
            // Get the currently stored address and store it.
            let pml4_addr = crate::arch::registers::get_cr3() & !0xFFF;
            self.table_addr = Some(pml4_addr);
            oxid_log!("Initializing the kernel page table. PML4 is at 0x{:x}.", pml4_addr);
            
            // Get the table from the address, and set the self-reference entry.
            let mut table = PML4::at(pml4_addr);
            table[SELF_ENTRY_IDX].set_addr(pml4_addr);
            table[SELF_ENTRY_IDX].set_present(true);
            table[SELF_ENTRY_IDX].set_writable(true);
        }
    }
    
    /// A function which checks if a given address is in canonical form. This has to be the case 
    /// in this architecture (this assumes the 48-bit address width).
    ///
    /// # Parameters
    /// `addr` : The address which we're checking.
    ///
    /// # Returns
    /// true if the address is valid and in canonical form, false otherwise.
    #[inline(always)]
    pub fn is_canonical(addr: usize) -> bool {
        // More details can be found at:
        // https://stackoverflow.com/questions/25852367/x86-64-canonical-address
        addr <= 0x00007FFFFFFFFFFF || addr >= 0xFFFF800000000000 
    }
    
    /// A function which maps a given page address (starting address) to a given frame address. It 
    /// additionally sets the required permissions and creates new tables if needed.
    ///
    /// # Parameters
    /// `page_addr` : The starting address of the wanted page.
    /// `frame_addr` : The starting address of the frame we're mapping the page to.
    /// `is_user` : True if the permissions are user accessible, False otherwise.
    /// `is_writable` : True if R/W, False if it's read-only.
    /// `is_no_exec` : True if not executable, False otherwise.
    ///
    /// # Returns
    /// Ok if everything went as expected, Err otherwise.
    pub unsafe fn map(page_addr: usize, frame_addr: usize, is_user: bool, 
        is_writable: bool, is_no_exec: bool) -> Result<(), ()> {
        
        //oxid_log!("Mapping page 0x{:x} to frame 0x{:x}", page_addr, frame_addr);
        
        // Check if the passed page address is in canonical form. If not, return err.
        if ! PageTables::is_canonical(page_addr) {
            return Err(());
        }
                           
        // Get the indexes within each table.
        let pml4_idx = PML4::get_idx(page_addr);
        let pdp_idx = PDP::get_idx(page_addr);
        let pd_idx = PD::get_idx(page_addr);
        let pt_idx = PT::get_idx(page_addr);
        
        // Calculate each table's addressess.
        let pml4_addr = PML4_START_ADDR;
        let pdp_addr = PDP_START_ADDR | (pml4_idx << 12);
        let pd_addr = PD_START_ADDR | (pml4_idx << 21) | (pdp_idx << 12);
        let pt_addr = PT_START_ADDR | (pml4_idx << 30) | (pdp_idx << 21) | (pd_idx << 12);
            
        // Get the pml4 from the self-reference entry, and create a PDP if not present in PML4.
        let mut pml4 = PML4::at(pml4_addr);
        pml4[pml4_idx].make_table_if_not_present(pdp_addr, is_user, is_writable, is_no_exec)?;
        
        // Get the pdp from the self-reference entry, and create a PD if not present in PDP.
        let mut pdp = PDP::at(pdp_addr);
        pdp[pdp_idx].make_table_if_not_present(pd_addr, is_user, is_writable, is_no_exec)?;
        
        // Get the pd from the self-reference entry, and create a PT if not present in PDP.
        let mut pd = PD::at(pd_addr);
        pd[pd_idx].make_table_if_not_present(pt_addr, is_user, is_writable, is_no_exec)?;
        
        // Get the page table from the self-reference entry.
        let mut pt = PT::at(pt_addr);
        pt[pt_idx] = pt::PTEntry::new();
        pt[pt_idx].set_present(true);
        pt[pt_idx].set_addr(frame_addr);
        pt[pt_idx].set_user(is_user);
        pt[pt_idx].set_writable(is_writable);
        pt[pt_idx].set_no_execute(is_no_exec);
        
        // If we get here, everything wnt as expected and the address is mapped.
        Ok(())
    }
    
    /// A function which unmaps a given page address. It resets the lowest level page table entry 
    /// for this specific page and "frees" it for future use. It does not remove the page table 
    /// however since it is likely that it will be re-used and it doesn't take that much space.
    ///
    /// # Parameters
    /// `page_addr` : The address of the page which we're unmapping.
    ///
    /// # Returns
    /// Ok if the given page was unmapped, Err if invalid address or non-existant page.
    pub unsafe fn unmap(page_addr: usize) -> Result<(), ()> { 
        // Get a pointer to the page table entry and check for errors. If anty error occured, 
        // simply return Err, otherwise reset the entry to a new (unpresent) one, and return ok.
        match PageTables::get_pt_entry_ptr(page_addr) {
            Ok(entry_ptr) => { 
                (*entry_ptr) = pt::PTEntry::new();      // Reset entry to a new one. 
                super::tlb::invalidate(page_addr);      // Invalidate it in TLB.
                Ok(()) 
            },
            
            Err(()) => Err(())
        }
    }
    
    /// A function which translates a given virtual address to it's corresponding physical address 
    /// based on the currently stored page table. It will return an Err if the page table is not 
    /// set-up or if the address is not currently mapped.
    ///
    /// # Parameters
    /// `page_addr` : The address of the page which we're translating.
    ///
    /// # Returns
    /// Ok(frame_addr) if everything went as expected, Err otherwise.
    pub fn virt_to_phys(page_addr: usize) -> Result<usize, ()> {
        unsafe {
            // Get a pointer to the page table entry and check for errors. If anty error occured, 
            // simply return Err, otherwise (addr is present) save the offset and return it.
            match PageTables::get_pt_entry_ptr(page_addr) {
                Ok(entry_ptr) => Ok((*entry_ptr).get_addr() | (page_addr & 0xFFF)),
                Err(()) => Err(())
            }
        }
    }
    
    /// A method which loads this page table into the system (using the CR3 register). It keeps 
    /// the properties that were previously stored in the CR3 register.
    pub unsafe fn load(&self) {
        // Get the bitmasks to seperate the address and properties section of CR3. For more details
        // please look at https://wiki.osdev.org/CPU_Registers_x86-64#CR3
        const CR3_PROPERTIES_BITMASK: usize = 0xFFF;
        const CR3_ADDR_BITMASK: usize = !CR3_PROPERTIES_BITMASK;
        
        // Get the physical address of the table and make sure it's first 12 bits are empty.
        let mut pml4_addr = self.table_addr.unwrap();
        pml4_addr &= CR3_ADDR_BITMASK;
        
        // Get the current CR3 register and clear it's address section.
        let mut curr_cr3 = crate::arch::registers::get_cr3();
        curr_cr3 &= CR3_PROPERTIES_BITMASK;
        
        // Set the new value of CR3 register to the new address with old properties.
        // in this architecture, this will automatically flush the TLB (all of it).
        crate::arch::registers::set_cr3(curr_cr3 | pml4_addr);
    }
    
    /// An internal function which tries to get a pointer to the page table (last level) entry which 
    /// is pointed to by the page_addr. If there is an issue with the passed address or if any of 
    /// the tables have a non-present entry, it will result in an err.
    ///
    /// # Parameters
    /// `page_addr` : The address of the page which we're trying to get an entry to.
    ///
    /// # Returns
    /// Ok(entry_ptr) if everything went as expected, Err if invalid or non-present address.
    unsafe fn get_pt_entry_ptr(page_addr: usize) -> Result<*mut pt::PTEntry, ()> {
        // Check if the passed page address is in canonical form. If not, return err.
        if ! PageTables::is_canonical(page_addr) {
            return Err(());
        }
        
        // Get the indexes within each table.
        let pml4_idx = PML4::get_idx(page_addr);
        let pdp_idx = PDP::get_idx(page_addr);
        let pd_idx = PD::get_idx(page_addr);
        let pt_idx = PT::get_idx(page_addr);
        
        // Calculate each table's addressess.
        let pml4_addr = PML4_START_ADDR;
        let pdp_addr = PDP_START_ADDR | (pml4_idx << 12);
        let pd_addr = PD_START_ADDR | (pml4_idx << 21) | (pdp_idx << 12);
        let pt_addr = PT_START_ADDR | (pml4_idx << 30) | (pdp_idx << 21) | (pd_idx << 12);
        
        // Get the pml4 table using the self reference entry, and check if entry is present.
        let pml4 = PML4::at(pml4_addr);
        if ! pml4[pml4_idx].is_present() {
            return Err(());
        }
        
        // Get the pdp table using the self reference entry, and check if entry is present.
        let pdp = PDP::at(pdp_addr);
        if ! pdp[pdp_idx].is_present() {
            return Err(());
        }
        
        // Get the pd table using the self reference entry, and check if entry is present.
        let pd = PD::at(pd_addr);
        if ! pd[pd_idx].is_present() {
            return Err(());
        }
        
        // Get the pt using the self reference entry, and check if entry is present.
        let mut pt = PT::at(pt_addr);
        if ! pt[pt_idx].is_present() {
            return Err(());
        }
        
        // If we get here, the address is present. So get a pointer to it and return it.
        Ok(&mut pt[pt_idx] as *mut pt::PTEntry)
    }
}


// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        // Run the entry tests.
        super::general_entry::test::run();
        
        // Make sure every entry is exactly 8 bytes.
        assert_eq!(core::mem::size_of::<super::pml_4::PML4Entry>(), 8);
        assert_eq!(core::mem::size_of::<super::pdp::PDPEntry>(), 8);
        assert_eq!(core::mem::size_of::<super::pd::PDEntry>(), 8);
        assert_eq!(core::mem::size_of::<super::pt::PTEntry>(), 8);
    }
}
