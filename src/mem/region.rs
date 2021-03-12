//! A structure which represents a single memory "region". Each region is simply an address with 
//! a size attribute. It additionally provides some helper functionality.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

/// A structure which represents a memory region.
#[derive(Copy, Clone, Debug, Default)]
pub struct Region {
    pub addr: usize,                           // The starting address of the region.
    pub size: usize,                           // The total size of the region (in bytes).
}

impl Region {
    /// A constructor which creates a new region from a start and end address. It calcualtes the 
    /// size and simply sets it. The start address should be lower than the end address.
    ///
    /// # Parameters
    /// `start_addr` : The starting address of this region (lower).
    /// `end_addr` : The ending address of this region (higher).
    ///
    /// # Returns
    /// The newly created region with the calculated size.
    pub fn new(start_addr: usize, end_addr: usize) -> Self {
        // Make sure the addresses are correct.
        assert!(start_addr <= end_addr);
        
        // Calculate the size and return the new region.
        Region {
            addr: start_addr,
            size: (end_addr - start_addr),
        }
    }
    
    /// A simple default constructor for the region with an address and a size. In this case, it 
    /// literally just sets addr and size and returns a new region.
    ///
    /// # Parameters
    /// `addr` : The starting of this region.
    /// `size` : The number of bytes in this region.
    ///
    /// # Returns
    /// The newly created region with the given properties.
    pub fn new_sized(addr: usize, size: usize) -> Self {
        Region {
            addr: addr,
            size: size,
        }
    }
    
    /// A constructor which creates a new aligned region from a start and end address. It 
    /// aligns the start_addr to become higher, and aligns the end_addr to be lower, it then 
    /// calculates the size and sets it. The start address should be lower than the end address.
    ///
    /// # Parameters
    /// `start_addr` : The starting address of this region (lower), it will be aligned higher.
    /// `end_addr` : The ending address of this region (higher), it will be aligned lower.
    /// `alignment` : The alignment for this region.
    ///
    /// # Returns
    /// The newly created region with the calculated size.
    pub fn new_aligned(start_addr: usize, end_addr: usize, alignment: usize) -> Self {
        // Align the start and end addreses.
        let aligned_start_addr: usize = super::align::align_higher(start_addr, alignment);
        let aligned_end_addr: usize = super::align::align_lower(end_addr, alignment);
        
        // Make sure the addresses are correct.
        assert!(aligned_start_addr <= aligned_end_addr);
        
        // Calculate the size and return the new region.
        Region {
            addr: start_addr,
            size: (end_addr - start_addr),
        }
    }
    
    /// A simple getter for the ending address of this region.
    ///
    /// # Returns
    /// The ending address of this region
    #[inline]
    pub fn end_addr(&self) -> usize {
        self.addr + self.size
    }
    
    /// A method which determines if this region includes a given address or not.
    ///
    /// # Parameters
    /// `addr` : The address which we're checking.
    ///
    /// # Returns
    /// True if the addr is within this region. False otherwise.
    #[inline]
    pub fn includes(&self, addr: usize) -> bool {
        self.addr >= addr && self.addr < self.end_addr()
    }
    
    /// A method which determines if this region can include another region inside of it.
    ///
    /// # Parameters
    /// `other_region` : The region which we're trying to fit inside this one.
    ///
    /// # Returns
    /// True if it can fit, False otherwise.
    #[inline]
    pub fn fits(&self, other_region: &Region) -> bool {
        // Make sure it includes the start and end addresses of other_region.
        self.includes(other_region.addr) 
            && self.includes(other_region.end_addr())
    }
}
