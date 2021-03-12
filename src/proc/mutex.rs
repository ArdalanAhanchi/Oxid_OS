//! A module which implements a basic mutex. It is essentially a wrapper for a spinlock which is 
//! implemented in assembly and uses the hardware compare and swap (cmpxchg) instruction.
//! The basic design is based on https://wiki.osdev.org/Synchronization_Primitives (in C).
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

#![allow(dead_code)]        // So we don't have to use it.

const UNLOCKED: u8 = 0;     // Represents the unlocked state.
const LOCKED: u8 = 1;       // Represents the locked state.

/// A structure which represents a simple mutex. It and allows safe access to critical sections.
#[repr(C, packed)]
pub struct Mutex {
    state: u8,              // Determines if the current status is locked or not.
}

impl Mutex {
    /// The main constructor which simple initializes a default mutex (unlocked), and stores
    /// the data. 
    ///
    /// # Returns
    /// The created mutex type which encloses the type passed.
    pub const fn new() -> Self {
        // Create a new mutex which is unlocked by default.
        Mutex {
            state: UNLOCKED,
        }
    }
    
    /// A method which locks this mutex. It utilizes a spinlock which is implemented in assmebly 
    /// (since it is more efficient). The implementation can be found at the arch/proc/sync.asm file.
    ///
    /// # Returns
    /// A mutable reference to the enclosed type.
    pub fn lock(&mut self) {
        // The wrapped hardware instructions are inherently unsafe.
        unsafe {
            // Disable interrupts to avoid context switching while mutex is locked.
            crate::arch::interrupts::disable();
        
            // Call the spinlock to perform the action.
            crate::arch::proc::sync::spin_lock(&mut self.state as *mut u8);
        }
    }
    
    /// A function which unlocks a mutex, it allows other threads to access it as well.
    /// it simply changes the locked variable in the struct.
    pub fn unlock(&mut self) {
        self.state = UNLOCKED;
        
        // Re-enable interrupts to allow switching.
        unsafe { crate::arch::interrupts::enable(); }
    }
}

