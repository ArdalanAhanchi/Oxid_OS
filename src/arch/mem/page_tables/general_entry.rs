//! A file which defines a group of functions in a macro for a general table entry. It can be for 
//! any of the entries in any of the 4 tables. This only represents the definitions shared between
//! all the entry types. 
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(unused_macros)]

// Based on the AMD64 manual volume 2 in pages 135 and 142, the following functionality is shared 
// between all the entry types.
// [0] - P - Present - 0 is an unused entry, 1 is used.
// [1] - R/W - Read/Write - 0 is read-only page, 1 is read/write.
// [2] - U/S - User/Super - 0 is kernel mode, 1 is user programs.
// [3] - PWT - Page-level wirethrough - 0 uses a Writeback caching policy, 1 uses a Writethrough.
// [4] - PCD - Page-level cache disable - 0 makes the table cacheable, 1 is not.
// [5] - A - Accessed - 1 if the page was used.
// [12,51] - ADR - Address - The physical address of the frame pointed.
// [63] - NX - No execute - If set, no code can be executed here.

/// A macro which implements all the general functions for the entries. Please make sure that the 
/// passed entry is a struct wrapping a usize (which is 64-bits in this architecture).
macro_rules! impl_general_entry {
    ($curr_entry:ty) => {
        impl $curr_entry {
            #![allow(dead_code)]        // To allow partial usage.
            
            /// A default constant constructor which simply returns an empty entry.
            #[inline]
            pub const fn new() -> Self { Self(0) }
        
            /// A method which sets the present bit which means that it is used. It modifies the 
            /// bit 0 as specified by the architecture.
            ///
            /// # Parameters
            /// `is_present` : If true, the page is present. If false it is not.
            #[inline]
            pub fn set_present(&mut self, is_present: bool) {
                use crate::mem::bitwise::BitWise;
                self.0.write_bit(0, is_present);
            }
            
            /// A method which sets the writable bit which means that the page(s) can be written to.
            /// It modifies the bit 1 as specified by the architecture.
            ///
            /// # Parameters
            /// `is_writable` : If true the page can be written to, false is read-only.
            #[inline]
            pub fn set_writable(&mut self, is_writable: bool) {
                use crate::mem::bitwise::BitWise;
                self.0.write_bit(1, is_writable);
            }
            
            /// A method which sets the permissions of this page (table). Basically, it controlls
            /// if it is accessible to the kernel only or if it's available to the user as well.
            /// It modifies the bit 2 as specified by the architecture.
            ///
            /// # Parameters
            /// `is_user_accessible` : False is kernel mode, True is user programs.
            #[inline]
            pub fn set_user(&mut self, is_user_accessible: bool) { 
                use crate::mem::bitwise::BitWise;
                self.0.write_bit(2, is_user_accessible); 
            }
            
            /// A method which controls if the caching is writeback or write through (where the 
            /// writes are directly written to memory). 
            /// It modifies the bit 3 as specified by the architecture.
            ///
            /// # Parameters
            /// `is_writethrough` : False uses a Writeback caching policy, True uses a Writethrough.
            #[inline]
            pub fn set_writethrough(&mut self, is_writethrough: bool) {
                use crate::mem::bitwise::BitWise;
                self.0.write_bit(3, is_writethrough); 
            }
            
            /// A method which sets the cache disabled bit in the entry. It basically disables the 
            /// cache for the whole sub-address pointed to by this entry.
            /// It modifies the bit 4 as specified by the architecture.
            ///
            /// # Parameters
            /// `is_disabled` : If true cache will be disabled, false will make it enabled.
            #[inline]
            pub fn set_cache_disabled(&mut self, is_disabled: bool) {
                use crate::mem::bitwise::BitWise;
                self.0.write_bit(4, is_disabled); 
            }
            
            /// A method which sets the no execute bit in the entry. It doesn't allow any execution
            /// if this bit is set. It modifies the bit 63 as specified by the architecture.
            ///
            /// # Parameters
            /// `is_not_executable` : If true, no code pointed here can be executed.
            #[inline]
            pub fn set_no_execute(&mut self, is_not_executable: bool) {
                use crate::mem::bitwise::BitWise;
                self.0.write_bit(63, is_not_executable); 
            }
            
            /// A method which checks if the entry is currently present. It checks the bit 0
            /// as specified by the architecture.
            ///
            /// # Returns
            /// true if present, false otherwise.
            #[inline]
            pub fn is_present(&self) -> bool {
                use crate::mem::bitwise::BitWise;
                self.0.is_set(0)
            }
            
            /// A method which is a getter for the accessed bit set by the CPU. It checks the bit 5
            /// as specified by the architecture.
            ///
            /// # Returns
            /// true if this page was accessed, false otherwise.
            #[inline]
            pub fn was_accessed(&self) -> bool {
                use crate::mem::bitwise::BitWise;
                self.0.is_set(5)
            }
            
            /// A method which gets the address which is pointed to by this entry.
            ///
            /// # Returns
            /// The address stored in bits 12 to 51 of this entry.
            #[inline]
            pub fn get_addr(&self) -> usize {
                // Define the address bitmask which only allows the address to pass the filter.
                const ADDR_BITMASK: usize = 0x000F_FFFF_FFFF_F000;
                
                // Filter out the properties and only allow the adderss to get returned.
                self.0 & ADDR_BITMASK
            }
            
            /// A method which sets the address which is given as a parameter as the primary address
            /// that this entry points to.
            ///
            /// # Parameters
            /// `addr` : The address which we want to set as bits 12 to 51 (masked if longer).
            #[inline]
            pub fn set_addr(&mut self, addr: usize) {
                // Define the address bitmask which only allows the address to pass the filter.
                const ADDR_BITMASK: usize = 0x000F_FFFF_FFFF_F000;
                
                // Filter out the address currently stored.
                self.0 &= !ADDR_BITMASK;
                
                // Make sure the passed address has the properties bits clear and set it.
                self.0 |= (addr & ADDR_BITMASK)
            }
        }
    }
}

/// A macro which allows an entry to create the next level table at the given position. It should be
/// implemented for PML4, PDP, and PD (since they all have a next level). The current entry type
/// and the pointed table name should be passed to this macro.
macro_rules! impl_make_table_if_not_present {
    ($curr_entry:ty, $next_table_type: tt) => {
        impl $curr_entry {
            #![allow(dead_code)]        // To allow partial usage.
            
            /// A method which creates a table of the pointed_table type at this entry. It basically
            /// allocates a new frame using the frame allocator, resets all the entries to zero, 
            /// and creates the new table.
            ///
            /// # Parameters
            /// `table_virt_addr` : The virtual address of the new table we're creating.
            /// `is_user` : True if the permissions are for user mode. False if kernel mode.
            /// `is_writable` : True if it's R/W, False if read only.
            /// `is_no_exec` : True if not executable, False otherwise.
            ///
            /// # Returns
            /// Ok if successfully created, Err if frame allocator issue occured.
            pub unsafe fn make_table_if_not_present(&mut self, table_virt_addr: usize, 
                is_user: bool, is_writable: bool, is_no_exec: bool) -> Result<(), ()> {                
                // Check if the current is present or not.
                if ! self.is_present() {                
                    // Allocate a frame for the table and get it's address.
                    use crate::mem::frame_alloc::{alloc, FrameAllocResult};
                    let addr = match alloc() {
                        // If successful, store the address.
                        FrameAllocResult::Ok(addr) => addr,
                        // If failed, return err since we were unsuccessful.
                        _ => return Err(()),
                    };
                    
                    // Actually inintialize the table and set it's values.
                    self.set_present(true);
                    self.set_addr(addr);
                    self.set_writable(is_writable);
                    self.set_user(is_user);
                    self.set_no_execute(is_no_exec);
                    
                    // Initialize a new table at the given address.
                    $next_table_type::new(table_virt_addr);
                }
                
                // If we got here, everything went as planned. Return Ok.
                Ok(())
            } 
        }
    }
}

// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// Define a test entry with the correct type and use the defined macro on it.
    struct TestEnt(usize);
    impl_general_entry!(TestEnt);

    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        test_set_addr();
        test_get_addr();
    }
    
    /// Test the set_addr function.
    fn test_set_addr() {
        // Create a TestEnt struct with an initial 0 value.
        let mut ent = TestEnt(0);
        
        // Set an easily identifiable address.
        ent.set_addr(0xDEADBEEF123);
        
        // Check if it was successful.
        assert_eq!(ent.0, 0xDEADBEEF000);
        
        // Check a case where the passed address is larger (should truncate the address).
        ent = TestEnt(0);
        
        // Set an address larger than expected (it should remove the initial A).
        ent.set_addr(0xAABBCCDDEEFF1000);
        
        // Check if it was successful.
        assert_eq!(ent.0, 0xBCCDDEEFF1000);
    }
    
    /// Test the get_addr function.
    fn test_get_addr() {
        // Create a TestEnt struct with an initial identifiable value.
        let ent = TestEnt(0xDEADBEEF123);

        // Get the address, and check if it was successful.
        assert_eq!(ent.get_addr(), 0xDEADBEEF000);
    }
}
