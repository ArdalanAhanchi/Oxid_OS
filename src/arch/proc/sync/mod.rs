//! An implementation of a hardware syncrhonization primitives for the X86-64 architecture. It uses 
//! the basic cmpxchg hardware instructions to implement it. The actual implementation can be found
//! in the sync.asm file. This is just a wrapper to allow rust access.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]        // To allow these to exist without actually calling them.

/// A group of signature definitions to allow calling assembly code from rust.
/// it utilizes the System V AMD64 calling conventions to call the assembly code.
extern "sysv64" {   
    /// A wrapper for the cmpxchg instruction available in the x86 architecture. This is the boolean
    /// version of it, and the asssembly implementation of this functino is implemented in the 
    /// src/arch/wrapper.asm file. The general algorithm is: 
    ///
    /// if *ptr == expected
    ///     *ptr = new;
    ///     return true;
    ///
    /// return false;
    ///     
    /// Explanation of the algorithm in: https://en.wikipedia.org/wiki/Compare-and-swap
    ///
    /// # Parameters
    /// `ptr` : The pointer to where the dst is for cmpxchg.
    /// `expected` : The value which we compare against so we can set the new value.
    /// `new` : The new value which will be set when the value in ptr is set to the expected.
    ///
    /// # Returns
    /// true if the value is set to new, false if it didn't.
    pub fn atomic_bool_lock_cmpxchg(ptr: *const bool, expected: bool, new: bool) -> bool;
    
    
    /// An implementation of a fairly performant spin lock mechanism. It's definition
    /// is based on the sysv64 ABI. It accepts a pointer to an integer (or boolean)
    /// value. And locks until the value becomes 0.
    ///
    /// # Parameters
    /// `ptr` : The value which we're locking on.
    pub fn spin_lock(ptr: *mut u8);
}

