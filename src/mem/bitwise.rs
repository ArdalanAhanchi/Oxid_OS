//! A submodule which implements bitwise operations for all the unsigned integer types. It provides
//! syntactic sugar for bitwise operations.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

/// A trait which when implemented, allows the bitwise operations on the various types.
pub trait BitWise {
    /// A method which checks if a given bit is set (1).
    ///
    /// # Parameters
    /// `bit_num` : The number of the bit which we're checking (starts from 0).
    fn is_set(&self, bit_num: usize) -> bool;
    
    /// A method which checks if a given bit is clear (0).
    ///
    /// # Parameters
    /// `bit_num` : The number of the bit which we're checking (starts from 0).
    fn is_clear(&self, bit_num: usize) -> bool;
    
    /// A function which simply sets the bit at bit_num (it will become 1).
    ///
    /// # Parameters
    /// `bit_num` : The number of the bit which we're modifying (starts from 0).
    fn set_bit(&mut self, bit_num: usize);
    
    /// A method which simply clears the bit at bit_num (it will become 0).
    ///
    /// # Parameters
    /// `bit_num` : The number of the bit which we're modifying (starts from 0).
    fn clear_bit(&mut self, bit_num: usize);
    
    /// A method which sets the value of the given bit_num to a passed boolean.
    ///
    /// # Parameters
    /// `bit_num` : The number of the bit which we're modifying (starts from 0).
    /// `value` : False will unset the bit, true will set it.
    fn write_bit(&mut self, bit_num: usize, value: bool);
}

/// A macro which automatically implements the bitwise operators for a given type.
///
/// # Parameters
/// The only parameter is the type which we're implementing the BitWise trait for.
macro_rules! impl_bitwise {
    ($curr_type:ty) => {
        /// Implement BitWise operations for a given type.
        /// For function header comments, refer to the BitWise trait's comments.
        impl BitWise for $curr_type {
            #[inline(always)]
            fn is_set(&self, bit_num: usize) -> bool { !self.is_clear(bit_num) }
            
            #[inline(always)]
            fn is_clear(&self, bit_num: usize) -> bool { *self & (1 << bit_num) == 0 }
            
            #[inline(always)]
            fn set_bit(&mut self, bit_num: usize) { *self |= 1 << bit_num }
            
            #[inline(always)]
            fn clear_bit(&mut self, bit_num: usize) { *self &= !(1 << bit_num) }
            
            #[inline(always)]
            fn write_bit(&mut self, bit_num: usize, value: bool) {
                // If the given value is true, set the bit, otherwise clear it.
                if value {
                    self.set_bit(bit_num);
                } else {
                    self.clear_bit(bit_num);
                }
            }
        }
    }
}

// Implement the bitwise trait for all the unsigned integer types in rust.
impl_bitwise!(u8);
impl_bitwise!(u16);
impl_bitwise!(u32);
impl_bitwise!(u64);
impl_bitwise!(u128);
impl_bitwise!(usize);


// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    use crate::mem::bitwise::BitWise;

    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        test_is_set();
        test_is_clear();
        test_set_bit();
        test_clear_bit();
        test_write_bit();
    }
    
    /// Unit tests for the is_set function.
    fn test_is_set() {    
        // Check an obvious one where the second bit is set.
        let test_one: usize = 0b10;
        assert!(test_one.is_set(1));
        assert!(!test_one.is_set(0));
        
        // Check one where the second bit is free.
        let test_two: usize = 0b0100;
        assert!(!test_two.is_set(3));
        assert!(test_two.is_set(2));
    }
    
    /// Unit tests for the is_clear function.
    fn test_is_clear() {
        // Check an obvious where the third and fifth bits are set.
        let test_one: usize = 0b10100;
        assert!(!test_one.is_clear(4));
        assert!(test_one.is_clear(1));
        
        // Check one where the second bit is free.
        let test_two: usize = 0b100;
        assert!(!test_two.is_clear(2));
        assert!(test_two.is_clear(12));
    }
    
    /// Unit tests for the set_bit function.
    fn test_set_bit() {
        // Set the 13th bit and check the results.
        let mut test_one: usize = 0;
        test_one.set_bit(12);
        assert_eq!(test_one, 0b1000000000000);
    }
    
    /// Unit tests for the clear_bit function.
    fn test_clear_bit() {
        // Free the 10th bit and check the results.
        let mut test_one: usize = 0b1111111111;
        test_one.clear_bit(9);
        assert_eq!(test_one, 0b0111111111);
        
        // Free the 2nd bit and check the results.
        let mut test_two: usize = 0b1111111111;
        test_two.clear_bit(1);
        assert_eq!(test_two, 0b1111111101);
    }
    
    /// Unit tests for the write_bit function.
    fn test_write_bit() {
        // Write a 1 into the 4th bit and check the results.
        let mut test_one: usize = 0;
        test_one.write_bit(3, true);
        assert_eq!(test_one, 0b1000);
        
        // Write a 0 into the 4th bit and check the results.
        let mut test_two: usize = 0b1111111111;
        test_two.write_bit(3, false);
        assert_eq!(test_two, 0b1111110111);
    }
}
