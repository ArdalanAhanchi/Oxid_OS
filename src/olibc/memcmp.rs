//! The implementation of the memcmp function of the C library as specified. It is used to allow
//! running the rust core library.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

// TODO: Optimize (vectored operations).

#![no_builtins]

/// Compares "num" bytes from the value stored in "first" to the "second".
///
/// # Parameters
/// `first` : A constant pointer which we're trying to compare.
/// `second` : A constant pointer which we're trying to compare.
/// `num` : The number of bytes which we're trying to compare.
///
/// # Returns
/// Positive if first is larger, negative if smaller, and zero if they are equal.
#[no_mangle]
pub unsafe extern fn memcmp(first: *const u8, second: *const u8, num: usize) -> i32 {
    // TODO: Compare longs first and then bytes (proper allignment).

    // Go through all the bytes required.
    // Rust for loops can not be used in this enviornment yet, so use a while.
    let mut i = 0;
    while i < num {
        // Dereference the current byte and save them.
        let first_deref: u8 = *first.offset(i as isize);
        let second_deref: u8 = *second.offset(i as isize);
        
        // Check if the values aren't equal, return their difference.
        if first_deref != second_deref {
            return (first_deref as i32) - (second_deref as i32);
        }
        
        i += 1;
    }
    
    0          // If they are equal, return a zero. 
}
