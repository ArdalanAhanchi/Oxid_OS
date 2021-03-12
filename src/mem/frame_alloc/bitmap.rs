//! A submodule which represents a bitmap frame allocator, and it initializes the bitmap. 
//! It is quite unsafe since at this point, there is no memory manager, and everything happens 
//! through unsafe interfaces.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

use crate::mem::bitwise::BitWise;   // To allow setting or reading bit by bit.
use crate::mem::region::Region;       // To get and utilize memory regions.

/// Represents how many bits are present in each bit field.
const NUM_BITS_PER_FIELD: usize = core::mem::size_of::<usize>() * 8;

/// A structure which represents the main bitmap. It will be initialized by a starting address of 
/// the free memory, and the size of it. It will use as many bits as necessary to manage the bits.
pub struct BitMap {
    frames_start: usize,            // The starting address of the first allocatable frame.
    frames_count: usize,            // The number of frames managed by this bit field.
    map: &'static mut [usize],      // The actual bit field utilized.
    first_free: usize,               // The first map idx which has a free bit (optimization).
}

/// An enum which represents the results for the allocation of the bitmap allocator.
pub enum BitMapResult {
    Allocated(usize),               // The result when the frame was successfuly allocated.
    AlreadyUsed,                    // The result when the wanted frame was already used.
    Full,                           // The result when there are no more free frames.
    InvalidFrameNum,                // When an invalid frame number was passed.
}

impl BitMap {
    /// A function which initializes a bitmap by getting a start address, and a size. This is 
    /// extremely unsafe since it assumes that the start address and whatever is within the range
    /// is accessible to use. Please make sure that memory addresses 0 to the address that you get 
    /// from the frame_alloc::get_first_frame_addr() are all identity mapped before calling.
    ///
    /// # Parameters
    /// `usable_region` : The area after the kernel where the frames will be allocated.).
    ///
    /// # Returns
    /// The newly created bitmap which should be already purged.
    pub unsafe fn new(usable_region: &Region) -> Self {
        // Create a new region with aligned start and end addresses.
        let aligned_usable_region = 
            Region::new_aligned(usable_region.addr, usable_region.end_addr(), super::ALIGNMENT);
        
        // Define the start and end addresses
        let aligned_start_addr: usize = aligned_usable_region.addr;
        let aligned_end_addr: usize = aligned_usable_region.end_addr();
    
        // Calculate the size of the bitmap (how many bytes we need to represent every frame).
        let mut bitmap_size: usize = ((aligned_end_addr - aligned_start_addr) 
            / super::FRAME_SIZE) / 8;
        
        // Check for when the number of frames is less than 8 (a byte).
        if bitmap_size == 0 {
            bitmap_size = 1;
        }
        
        // Calculate the address of the first allocatable physical frame while considering the 
        // bit map (which we know the size of right now).
        let start_frames: usize = 
            crate::mem::align::align_higher(aligned_start_addr + bitmap_size, super::ALIGNMENT);
            
        // Calculate how many frames we have considering the bitmap.
        let num_frames: usize = (aligned_end_addr - start_frames) / super::FRAME_SIZE;
        
        // Create and return the bitmap.
        let mut to_return = BitMap {
            frames_start: start_frames,         // Store the frame count, and start addr.
            frames_count: num_frames,           // Convert the raw pointer to a static slice.
            map: core::slice::from_raw_parts_mut(aligned_start_addr as *mut usize
                , num_frames / NUM_BITS_PER_FIELD),
            first_free: 0,                       // All is free now.
        };
        
        // Purge and return the newly created bitmap.
        to_return.purge();
        to_return
    }
    
    /// A function which purges the bitmap and basically deallocates all the memory. It does not 
    /// actually purge the memory, just the allocation bitmap of it. It should be called after the 
    /// kernel is identity mapped up to frames_start.
    pub fn purge(&mut self) {
        // Go through every item, and purge the fields.
        for i in 0..self.map.len() {
            self.map[i] = 0;
        }
        
        self.first_free = 0;
    }
    
    /// A method which finds the first available free frame, and allocates it. It then returns the 
    /// frame number associated with the allocated frame.
    ///
    /// # Returns
    /// BitMapResult::Allocated(frame_number) if successful. BitMapResult::Full otherwise.
    pub fn alloc(&mut self) -> BitMapResult {
        // Go through every bit field (Starting from last free), and check if it has an empty one.       
        for i in self.first_free..self.map.len() {
            // Check if even one of the bits is empty.
            if has_free(&self.map[i]) {
                // Get the empty bit's number. We already checked, so it should have one empty.
                let free_bit_num = get_free(&self.map[i]).unwrap();
                
                // Set the free bit to used.
                self.map[i].set_bit(free_bit_num);
                
                // Store the new first free index (since we searched to find it).
                self.first_free = i;
                
                // Calculate the frame number for the given free bit, and return it.
                return BitMapResult::Allocated((i * NUM_BITS_PER_FIELD) + free_bit_num);
            }
        }
        
        // If we get here, there were no empty frames. So just return full.
        BitMapResult::Full
    }
    
    /// A method which specifically tries to allocate a specific frame number. It checks if it's 
    /// already allocated, and if not, it sets it to allocated. This is done to do specific mappings. 
    ///
    /// # Parameters
    /// `frame_num` : The number of the frame which we want to allocate.
    ///
    /// # Returns
    /// BitMapResult::Allocated(frame_number) if successful. 
    /// BitMapResult::AlreadyUsed if the frame was already used.
    /// BitMapResult::InvalidFrameNum if the passed frame number is out of range.
    pub fn alloc_frame_num(&mut self, frame_num: usize) -> BitMapResult {
        // Check if the frame_num is invalid.
        if frame_num >= self.frames_count {
            return BitMapResult::InvalidFrameNum;
        }
        
        // Calculate the idx within map where this frame is.
        let map_idx: usize = frame_num / NUM_BITS_PER_FIELD;
        
        // Calculate the specific bit number where this frame number is.
        let bit_num: usize = frame_num % NUM_BITS_PER_FIELD;
        
        // Check if the frame is already used. Otherwise, set it.
        if self.map[map_idx].is_clear(bit_num) {
            // Set it, and return the success result.
            self.map[map_idx].set_bit(bit_num);
            BitMapResult::Allocated(frame_num)
        } else {
            // If we get here, the frame number is already used.
            BitMapResult::AlreadyUsed
        }
    }
    
    /// A method which deallocates the given frame with the frame number. It simply resets
    /// the value in the bitfield corresponding to the frame number.
    ///
    /// # Parameters
    /// `frame_num` : The number of the frame which we want to deallocate.
    pub fn dealloc(&mut self, frame_num: usize) {
        // Check if the frame_num is valid.
        if frame_num < self.frames_count {
            // Calculate the index within the map.
            let map_idx = frame_num / NUM_BITS_PER_FIELD;
        
            // Calculate what is the idx within map, and the bit number, and then free it.
            self.map[map_idx].clear_bit(frame_num % NUM_BITS_PER_FIELD);
            
            // If the first free index is larger than map_idx, update it.
            if self.first_free > map_idx {
                self.first_free = map_idx;
            }
        }
    }
    
    /// A method which converts a given physical memory address to a frame number.
    ///
    /// # Parameters
    /// `addr` : The physical address which we want to convert.
    ///
    /// # Returns
    /// Result::Ok containing the frame number if everything went as expected.
    /// Result::Err if the address it not within range. 
    pub fn addr_to_frame(&self, addr: usize) -> Result<usize, ()> {
        // Check for invalid address passed (out of range).
        if addr < self.frames_start 
            || addr > self.frames_start + (self.frames_count * super::FRAME_SIZE) {
            Err(())                                                 // Error, invalid address.
        } else {        
            Ok((addr - self.frames_start) / super::FRAME_SIZE)      // Calculate the frame number.
        }
    }
    
    /// A method which converts a given frame number to it's starting physical address.
    ///
    /// # Parameters
    /// `frame_num` : The frame number which we want to get the address of.
    ///
    /// # Returns
    /// Result::Ok containing the address if everything went as expected.
    /// Result::Err if the frame number is out of range. 
    pub fn frame_to_addr(&self, frame_num: usize) -> Result<usize, ()> {
        // Check for invalid frame number passed (out of range).
        if frame_num < self.frames_count {
            Ok(self.frames_start + (frame_num * super::FRAME_SIZE)) // Find the addr and return it.
        } else {        
            Err(())                                                 // Error, invalid frame number.
        }
    }
    
    /// A function which calculates the region which is mappable by this bitmap. It starts at the 
    /// end of the bitmap (aligned), and ends at the end of the last frame. 
    ///
    /// # Returns
    /// A region which represents the usable memory area.
    pub fn get_mappable_region(&self) -> Region {
        Region::new_sized(self.frames_start, self.frames_count * super::FRAME_SIZE)
    }
}

/// A function which checks if a given bitfield (with the size of usize) has a free bit. This is 
/// used to compare 64 bits at a time (on a 64-bit machine). 
///
/// # Parameters
/// `bitfield` : The current chunk of the bitmap which we're comparing.
///
/// # Returns
/// True if there is at least one free bit, false otherwise.
#[inline]
fn has_free(bitfield: &usize) -> bool {
    *bitfield != usize::MAX                     // Compare with the largest value of usize.
}

/// A function which finds and returns the number of the free bit in the current bitfield. For 
/// example, if the first bit is set to free, it will just return 0.
///
/// # Parameters
/// `bitfield` : The current chunk of the bitmap which we're getting the free bit from.
///
/// # Returns
/// The bit number for the first available free bit. If none of them are free it returns None.
#[inline]
fn get_free(bitfield: &usize) -> Option<usize> {
    // Go through every single bit and check for free ones.
    for bit_num in 0..(core::mem::size_of::<usize>() * 8) {
        // Check if the particular bit is unused.
        if bitfield.is_clear(bit_num) {
            return Some(bit_num);
        }
    }
        
    None                          // If we get here, none of the bits were available/empty.
}


// Unit Tests **************************************************************************************

/// The main test function which will call all the unit tests. This code will only be compiled
/// in the test configuration. Add the unit-test feature to cargo to run all the unit tests. 
#[cfg(feature = "unit-test")]
pub mod test {
    /// The "main" function for the unit tests. It basically calls all the other unit tests in this 
    /// sub module. 
    pub fn run() {
        test_has_free();
        test_get_free();
    }
    
    /// Unit tests for the has_free function.
    fn test_has_free() {
        // Check an obvious value that should have a free bit.
        let test_one: usize = 0x010;
        assert!(super::has_free(&test_one));
        
        // Check one that shouldn't have a free bit.
        let test_two: usize = 0xFFFFFFFFFFFFFFFF;
        assert!(!super::has_free(&test_two))
    }
    
    /// Unit tests for the get_free function.
    fn test_get_free() {
        // Check it when the second bit is free.
        let test_one: usize = 0b01;
        assert_eq!(super::get_free(&test_one), Some(1));
        
        // Check it when none of the bits are free.
        let test_two: usize = 0xFFFFFFFFFFFFFFFF;
        assert_eq!(super::get_free(&test_two), None);
        
        // Check it when the 10th bit is free.
        let test_three: usize = 0b0111111111;
        assert_eq!(super::get_free(&test_three), Some(9));
    } 
}
