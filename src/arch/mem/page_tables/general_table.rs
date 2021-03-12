//! A file which defines a group of functions in a macro for a general table. It can be for 
//! any of the 4 tables. This only represents the definitions shared between all the tables. 
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(unused_macros)]

/// A macro which implements all the general functions for the table. Please make sure that the 
/// first passed parameter is the table struct, and second is it's corresponding entry struct.
macro_rules! impl_general_table {
    ($curr_table:tt, $entry_type:tt) => {
        // Implement some general functions.
        impl $curr_table {
            #![allow(dead_code)]        // To allow partial usage.
            
            /// A default constructor which creates a new table at a given (virtual) address. It
            /// initializes every element to their defualt values.
            ///
            /// # Parameters
            /// `addr` : The virtual address where the table is located at.
            ///
            /// # Returns
            /// A new table with every element set to their default value.
            pub fn new(addr: usize) -> Self {
                // Create a new table with the given address, and set the number of entries.
                let mut table: $curr_table = $curr_table {
                    virt_addr: addr,
                    num_entries: super::NUM_ENTRIES,
                };
                
                // Reset every entry to their default value.
                for i in 0..table.num_entries {
                    table[i] = $entry_type::new();
                }
                
                table
            }
            
            /// A constructor which creates a table object at a given address (interprets it as a 
            /// table). The address should be accessible (virtual). It does not override the 
            /// contents of the table.
            ///
            /// # Parameters
            /// `addr` : The address where the table is located at.
            ///
            /// # Returns
            /// An existing table which can be indexed into.
            #[inline]
            pub unsafe fn at(addr: usize) -> Self {
                // Initialize and return a table at the given address. 
                $curr_table {
                    virt_addr: addr,
                    num_entries: super::NUM_ENTRIES,
                }
            }
            
            /// A function which gets the address of this table and returns it.
            ///
            /// # Returns
            /// The starting address of the table.
            #[inline]
            pub fn get_addr(&self) -> usize {
                self.virt_addr
            }
        }
        
        /// Implement the index trait to be able to access elements directly.
        impl core::ops::Index<usize> for $curr_table {
            type Output = $entry_type;
            
            /// A function which is an accessor for the array enclosed.
            #[inline(always)]
            fn index(&self, index: usize) -> &$entry_type {
                // Make sure the index is within range.
                assert!(index < self.num_entries);
            
                // Calculate the address of the wanted entry, and return a reference to it.
                let addr = (self.virt_addr + (core::mem::size_of::<$entry_type>() * index));
                unsafe { &*(addr as *const $entry_type) }
            }
        }
        
        /// Implement the mutable index trait to be able to modify elements directly.
        impl core::ops::IndexMut<usize> for $curr_table {
            /// A function which is a mutable accessor for the array enclosed.
            #[inline(always)]
            fn index_mut(&mut self, index: usize) -> &mut $entry_type {
                // Make sure the index is within range.
                assert!(index < self.num_entries);
            
                // Calculate the address of the wanted entry, and return a mutable reference to it.
                let addr = (self.virt_addr + (core::mem::size_of::<$entry_type>() * index));
                unsafe { &mut*(addr as *mut $entry_type) }
            }
        }
    }
}
