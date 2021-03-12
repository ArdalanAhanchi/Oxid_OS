//! The implementation of the memcpy function of the C library as specified. It is used to allow
//! running the rust core library.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

// TODO: Optimize (vectored operations).

#![no_builtins]

/// Copies "num" bytes (characters) from memory location "src" to "dst". Compared to memcpy, the 
/// the addresses here may overlap.
///
/// # Parameters
/// `dst` : A mutable pointer to the destination memory area.
/// `src` : A non-mutable pointer to the source memory area (to read from).
/// `num` : The number of bytes which we're trying to copy.
///
/// # Returns
/// A pointer to the destination (the same as dst).
#[no_mangle]
pub unsafe extern fn memmove(dst: *mut u8, src: *const u8, num: usize) -> *mut u8 {    
    // TODO: Copy longs first and then bytes (proper allignment), more complex __np_anyptrlt.
    //crate::olibc::memcpy::memcpy(dst, src, num)
    
    // Check if the src is smaller than dst (naive implementatino of __np_anyptrlt).
    if (src as usize) < (dst as usize) {
        // Rust for loops can not be used in this enviornment yet, so use a while.
        let mut i = num;
        while i != 0 {
            // Go through all bytes, and copy backward the actual data.
            i -= 1;
            *(((dst as usize) + i) as *mut u8) = *(((src as usize) + i) as *mut u8);
        }
    } else {
        // Rust for loops can not be used in this enviornment yet, so use a while.
        let mut i = 0;
        while i < num {
            // Go through all bytes, and copy forward the actual data.
            *(((dst as usize) + i) as *mut u8) = *(((src as usize) + i) as *mut u8);
            i += 1;
        }
    }
    
    dst
}
