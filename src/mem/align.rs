//! A module which allows for memory alignment to lower and higher values if needed.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

/// A function which aligns a given address to the closest address which is higher than what the
/// address is right now. It can be used for dynamic memory alignment. More explanation can be 
/// found at: https://embeddedartistry.com/blog/2017/02/22/generating-aligned-memory/
///
/// # Parameters
/// `addr` : The address which we want to align.
/// `alignment` : The alignment which we're looking for.
///
/// # Returns
/// The properly aligned address. It will be equal or higher than the passed addr.
#[inline]
pub fn align_higher(addr: usize, alignment: usize) -> usize {
    (addr + (alignment - 1)) & !(alignment - 1)
}

/// A function which aligns a given address to the closest address which is lower than what the
/// address is right now. It can be used for dynamic memory alignment.
///
/// # Parameters
/// `addr` : The address which we want to align.
/// `alignment` : The alignment which we're looking for.
///
/// # Returns
/// The properly aligned address. It will be equal or lower than the passed addr.
#[inline]
pub fn align_lower(addr: usize, alignment: usize) -> usize {
    addr - (addr % alignment)
}

// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        test_align_higher();
        test_align_lower();
    }
    
    /// Test the align higher function.
    fn test_align_higher() {
        // Check higher alignment.
        let test_one = 0x1001;
        let aligned_t_one = super::align_higher(test_one, 0x1000);
        assert_eq!(aligned_t_one, 0x2000);
        
        // Check with already aligned value.
        let test_two = 0x1000;
        let aligned_t_two = super::align_higher(test_two, 0x1000);
        assert_eq!(aligned_t_two, 0x1000);
        
        // Check with zero.
        let test_three = 0x0000;
        let aligned_t_three = super::align_higher(test_three, 0x1000);
        assert_eq!(aligned_t_three, 0x0000);
    }
    
    /// Test the align lower function.
    fn test_align_lower() {
        // Check lower alignment.
        let test_one = 0x1010;
        let aligned_t_one = super::align_lower(test_one, 0x1000);
        assert_eq!(aligned_t_one, 0x1000);
        
        // Check with already aligned value.
        let test_two = 0x1000;
        let aligned_t_two = super::align_higher(test_two, 0x1000);
        assert_eq!(aligned_t_two, 0x1000);
        
        // Check with zero.
        let test_three = 0x0000;
        let aligned_t_three = super::align_higher(test_three, 0x1000);
        assert_eq!(aligned_t_three, 0x0000);
    }
}
