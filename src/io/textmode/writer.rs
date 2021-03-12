//! A writer for the textmode driver which allows buffered writes to the driver. It handles 
//! new lines, and in general allows printing of strings. It is also thread safe, so it can be 
//! shared by multiple processes.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

#![allow(dead_code)]              // Because we might not use all functions or attributes.

use crate::io::textmode::{driver::Driver, color::Color};        // To allow accessing the driver.
use crate::proc::mutex::Mutex;                            // To allow synchronization.
use core::fmt;

const DEFAULT_BG_COLOR: Color = Color::Black;                   // Default background color.
const DEFAULT_FG_COLOR: Color = Color::Green;                   // Default foreground color (Text).

/// A struct which abstracts over different types of writers, and allows writing of a group of 
/// characters to the textmode driver.
pub struct Writer<T: Driver> {
    vga_driver: T,              // Hold the driver for the textmode.
    cursor_row: usize,          // Holds the current row the writer's cursor is at.
    cursor_col: usize,          // Holds the current column the writer's cursor is at.
    curr_fg: Color,             // The current foreground color.
    curr_bg: Color,             // The current background color.
    mutex: Mutex,               // To allow safe access.
}

impl<T: Driver> Writer<T> {
    /// Primary constructor which creates a vga buffer with the default rows and columns.
    /// it then clears the screen completely, and initializes the cursor row and column.
    /// it additionally uses the default colors.
    ///
    /// # Parameters
    /// `vga_driv` : The vga driver which is used to initialize this writer.
    ///
    /// # Returns
    /// The newly created textmode writer which the user can print into.
    pub fn new(vga_driv: T) -> Self {
        let mut new_writer: Writer<T> = Writer {                         // Initialize a writer.
            vga_driver: vga_driv, 
            cursor_row: 0, 
            cursor_col: 0,
            curr_fg: DEFAULT_FG_COLOR,
            curr_bg: DEFAULT_BG_COLOR,
            mutex: Mutex::new(),
         };  
        
        new_writer.clear();                                             // Clear the terminal.
        new_writer                                                      // Return the new writer.
    }
    
    /// A function which writes a string to the buffer. It can also write it in configurable
    /// background and foreground colors. It can also print new lines and shift down when needed.
    ///
    /// # Parameters
    /// `string` : The actual string we want to print.
    /// `fg` : The foreground color for the printed string.
    /// `bg` : The background color for the printed string.
    pub fn print_colored(&mut self, string: &str, fg: Color, bg: Color) {
        // Lock the mutex to allow safe modification access.
        //self.mutex.lock();
    
        // Go through every byte in the string.
        for character in string.bytes() {
            // If we have a newline character, go to the next line.
            if character == b'\n' {
                self.new_line();
            // Otherwise, set the cell at the current cursor position.    
            } else {
                unsafe {
                    // Set the cell at the current cursor row and column. 
                    self.vga_driver.set_cell(character, fg, bg, self.cursor_row, self.cursor_col);
                    
                    // Increase the column.
                    self.cursor_col += 1;
                    
                    // If the cursor is out of the screen, add a new line.
                    if self.cursor_col >= self.vga_driver.get_cols() {
                        self.new_line();
                    }
                }   
            }    
        }
        
        // Unlock the mutex since we're done with the modifications.
        self.mutex.unlock();
    }
    
    /// A wrapper for the print_colored function which prints in the default colors. It only accepts
    /// the string which we want to print.
    ///
    /// # Parameters
    /// `string` : The actual string we want to print.
    #[inline]
    pub fn print(&mut self, string: &str) {
        self.print_colored(string, self.curr_fg, self.curr_bg);
    }
    
    /// A function which sets the current color which will be used by the print method. All the
    /// future writes will be with the given colors.
    ///
    /// # Parameters
    /// `fg` : The foreground color we want to set.
    /// `bg` : The background color we want to set.
    #[inline]
    pub fn set_colors(&mut self, fg: Color, bg: Color) {
        // Lock the mutex to allow safe modification access.
        //self.mutex.lock();
    
        self.curr_fg = fg;
        self.curr_bg = bg;
        
        // Unlock the mutex since we're done with the modifications.
        self.mutex.unlock();
    }
    
    /// A function which resets the colors to their default color mode.
    #[inline]
    pub fn reset_colors(&mut self) {
        self.set_colors(DEFAULT_FG_COLOR, DEFAULT_BG_COLOR);
    }

    /// A method which clears the whole console screen. It basically ends up with an empty canvas
    /// to write on. It should be called when initializing the screen.
    pub fn clear(&mut self) {
        // Lock the mutex to allow safe modification access.
        //self.mutex.lock();
    
        // Go through every single row.
        let mut row : usize = 0;
        while row < self.vga_driver.get_rows() {
            self.clear_line(row);                   // Clear every column in the row.
            row += 1;
        }
        
        // Reset the row and column.
        self.cursor_row = 0;
        self.cursor_col = 0;
        
        // Unlock the mutex since we're done with the modifications.
        self.mutex.unlock();
    }
    
    /// A method which clears the last line in the console screen (current cursor row).
    pub fn clear_last_line(&mut self) {
        // Lock the mutex to allow safe modification access.
        //self.mutex.lock();
    
        // Simply clear the current line.
        self.clear_line(self.cursor_row);
        
        // Set the current column to 0.
        self.cursor_col = 0;
        
        // Unlock the mutex since we're done with the modifications.
        self.mutex.unlock();
    }
    
    /// A method which clears the last cell in the console screen (current cursor row, col).
    pub fn clear_last_cell(&mut self) {
        // Lock the mutex to allow safe modification access.
        //self.mutex.lock();
        
        // Get the row and column of previous cell.
        let (prev_row, prev_col) = self.prev_cell();
    
        // Simply clear the previous cell.
        unsafe { self.vga_driver.clear_cell(prev_row, prev_col) };
        
        // Set the current row and column.
        self.cursor_row = prev_row;
        self.cursor_col = prev_col;
        
        // Unlock the mutex since we're done with the modifications.
        self.mutex.unlock();
    }
    
    /// A method which clears a full line (row) in the console. It needs a row number (less than 
    /// the total number of rows) to clear it.
    ///
    /// # Parameters
    /// `row` : The number for the row which we want to clear.
    #[inline]
    fn clear_line(&mut self, row: usize) {
        // Go through every single column.
        let mut col: usize = 0;
        while col < self.vga_driver.get_cols() {
            // Clear each cell.
            unsafe { self.vga_driver.clear_cell(row, col); }   
            col += 1;
        }
    }
    
    /// A method which shifts everything up by one line. It also clears the last line (since the
    /// first line is now technically outside of the "view".
    fn shift_up(&mut self) {
        // Go through every row from 0 to the one before the last one.
        let mut row: usize = 0;
        while row < self.vga_driver.get_rows() - 1 {
            // Go through every single column.
            let mut col: usize = 0;
            while col < self.vga_driver.get_cols() {
                // Copy each cell to the one on top of it.
                unsafe { self.vga_driver.copy_cell(row + 1, col, row, col); }   
                col += 1;
            }
            row += 1;
        }
        
        // Clear the last line.
        self.clear_line(self.vga_driver.get_rows() - 1);
    }
    
    /// A function which adds a newline and shifts everything up if needed.
    #[inline]
    fn new_line(&mut self) {
        // Increase the row.
        self.cursor_row += 1;
        
        // If we're at the last line, shift everythin up, clear line, and fix at last row.
        if self.cursor_row >= self.vga_driver.get_rows() {
            self.shift_up();
            self.cursor_row = self.vga_driver.get_rows() - 1;
        }
        
        // Set the column to the first of the line.
        self.cursor_col = 0;
    }
    
    /// A method which calculates the previous cell from the current cursor position and returns 
    /// it's row and column number. If it reaches the beginning of the writer, it simply returns 
    /// the row and column of the first elelement.
    ///
    /// # Returns
    /// A tuple which represents the row and column (ordered).
    #[inline]
    fn prev_cell(&self) -> (usize, usize) {
        // Temporary value for the previous row.
        let mut prev_row: usize = self.cursor_row;
        
        // Calculate the previous column.
        let prev_col: usize = if self.cursor_col < 1 {
            // If the column can't be decreased, either return 0 if we're on the first row.
            if prev_row < 1 {
                prev_row = 0;
                0
            // Of go to the previous row and last column if there is a previous row.
            } else {
                prev_row -= 1;
                self.vga_driver.get_cols() - 1
            }
        } else {
            // If we're here, the column can be decreased.
            self.cursor_col - 1
        };
        
        // Return a tuple for row and column.
        (prev_row, prev_col)
    }
}

// To allow writing formatted text (For example, using format!). 
impl<T: Driver>  fmt::Write for Writer<T> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.print(string);
        Ok(())
    }
}

