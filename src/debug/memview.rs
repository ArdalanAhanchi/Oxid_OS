//! Some functions which allow dumping memory information to the console.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

const HEX_NUM_BYTES_PER_LINE: usize = 8;    // Number of bytes per line when printing in hex.
const BIN_NUM_BYTES_PER_LINE: usize = 4;    // Number of bytes per line when printing in binary.

/// A function which dumps the content of memory at location `addr` into the console
/// in hexadecimal format. Each line will contain 8 bytes.
///
/// # Parameters
/// `addr` : The address which the dump will start at.
/// `lines` : The number of lines (8 bytes each) which will be printed.
pub fn hex_dump(addr: usize, lines: usize) {
    oxid_println!("Printing memory dump in hexadecimal format.");

    // Go through every line.
    for line in 0..lines {
        // Calculate the address of the current line.
        let line_addr: usize = addr + (line * HEX_NUM_BYTES_PER_LINE);
    
        // Print the address of each line with a consistant format.
        oxid_print!("{:018p} : ", line_addr as *const u8);
        
        // Go through every byte, and print it with a consistant format (fixed size, filled).
        for byte in 0..HEX_NUM_BYTES_PER_LINE {
            oxid_print!("{:0>2x} ", *((line_addr + byte) as *const u8));
        }
        
        // Go to the next line.
        oxid_println!();
    }
}

/// A function which dumps the content of memory at location `addr` into the console
/// in binary format. Each line will contain 4 bytes.
///
/// # Parameters
/// `addr` : The address which the dump will start at.
/// `lines` : The number of lines (4 bytes each) which will be printed.
pub fn bin_dump(addr: usize, lines: usize) {
    oxid_println!("Printing memory dump in binary format.");

    // Go through every line.
    for line in 0..lines {
        // Calculate the address of the current line.
        let line_addr: usize = addr + (line * BIN_NUM_BYTES_PER_LINE);
    
        // Print the address of each line with a consistant format.
        oxid_print!("{:018p} : ", line_addr as *const u8);
        
        // Go through every byte, and print it with a consistant format (fixed size, filled).
        for byte in 0..BIN_NUM_BYTES_PER_LINE {
            oxid_print!("{:0>8b} ", *((line_addr + byte) as *const u8));
        }
        
        // Go to the next line.
        oxid_println!();
    }
}
