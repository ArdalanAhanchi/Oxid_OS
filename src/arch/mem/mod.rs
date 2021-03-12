//! A module which includes all the architecture dependent memory handling code. This includes
//! various paging structures, and their corresponding functionality.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

pub mod page_tables;
pub mod tlb;

// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        super::page_tables::test::run();
    }
}
