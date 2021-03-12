//! The implementation of the memcpy function of the C library as specified. It is used to allow
//! running the rust core library.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

// TODO: Optimize (vectored operations).

#![no_builtins]

/// Copies "num" bytes (characters) from memory location "src" to "dst".
///
/// # Parameters
/// `dst` : A mutable pointer to the destination memory area.
/// `src` : A non-mutable pointer to the source memory area (to read from).
/// `num` : The number of bytes which we're trying to copy.
///
/// # Returns
/// A pointer to the destination (the same as dst).
#[no_mangle]
pub unsafe extern fn memcpy(dst: *mut u8, src: *const u8, num: usize) -> *mut u8 {    
    // TODO: Copy longs first and then bytes (proper allignment).
    
    // Rust for loops can not be used in this enviornment yet, so use a while.
    let mut i = 0;
    while i < num {
        // Go through all bytes, and copy the actual data.
        *dst.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    
    dst
}
