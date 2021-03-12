//! The implementation of the memset function of the C library as specified. It is used to allow
//! running the rust core library.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

// TODO: Optimize (vectored operations).

#![no_builtins]

/// Sets the first byte of the "value" inside "num" bytes of the memory location
/// pointed to by the "dst".
///
/// # Parameters
/// `dst` : A mutable pointer to the destination memory area.
/// `value` : The value which we want to continously write to dst.
/// `num` : The number of bytes which we're trying to set.
///
/// # Returns
/// A pointer to the destination (the same as dst).
#[no_mangle]
pub unsafe extern fn memset(dst: *mut u8, value: i32, num: usize) -> *mut u8 {
    // Go through all the bytes required, and set the value.
    // Rust for loops can not be used in this enviornment yet, so use a while.
    let mut i = 0;
    while i < num {
        *dst.offset(i as isize) = value as u8;
        i += 1;
    }
    
    dst
}
