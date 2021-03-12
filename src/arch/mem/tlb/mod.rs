//! A sub-module which provides TLB management. In rust, it is just a wrapper to allow access to 
//! the assembly functions.
//! 
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021.

#![allow(dead_code)]

extern "sysv64" {
    /// A function which flushes every entry in TLB. It simply sets the CR3 register to what it was 
    /// initially stored in it.
    pub fn flush();

    /// A function which invalidates a specific address in TLB. It can be used in some situations 
    /// to avoid flushing every entry (much more efficient).
    ///
    /// # Parameters
    /// `page_addr` : The address of the page which we're invalidating in the TLB.
    pub fn invalidate(page_addr: usize);    
}
