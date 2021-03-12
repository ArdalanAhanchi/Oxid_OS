#![allow(dead_code)]        // To allow these to exist without actually calling them.

use crate::io::textmode::{driver::Driver, color::Color};

const DEFAULT_ROWS: usize = 25;         /// Default number of rows in the buffer.
const DEFAULT_COLS: usize = 80;         /// Default number of column in the buffer.
const VGA_BUFFER: usize = 0xb8000;      /// Memory location for the VGA buffer.

/// A structure which represents a textmode buffer for the x86_64 architecture. It keeps the 
/// rows and columns supported which by default are 80x25.
pub struct TextMode {
    rows: usize,
    cols: usize,
}

impl Driver for TextMode {
    /// Primary constructor which creates a vga buffer with a given row and column.
    /// it simply sets those values and returns it.
    ///
    /// # Parameters
    /// `rows` : The number of rows in the vga buffer.
    /// `cols` : The number of columns in the vga buffer.
    ///
    /// # Returns
    /// The newly created vga buffer.
    fn new(rows: usize, cols: usize) -> Self {
        TextMode {     // Initialize the struct and return it.
            rows,
            cols,
        }
    }

    /// Default constructor which creates a default 80x25 vga buffer.
    ///
    /// # Returns
    /// The newly created vga buffer.
    fn new_default() -> Self {
        TextMode::new(DEFAULT_ROWS, DEFAULT_COLS)
    }
    
    /// A public getter for the number of rows in the buffer.
    ///
    /// # Returns
    /// The number of rows in this object.
    fn get_rows(&self) -> usize {
        self.rows
    }
    
    /// A public getter for the number of cols in the buffer.
    ///
    /// # Returns
    /// The number of cols in this object.
    fn get_cols(&self) -> usize {
        self.cols
    }

    /// A method which sets a character in a cell at a row and column. 
    ///
    /// # Parameters
    /// `character` : The ASCII byte which we want to set.
    /// `fg` : The foreground color (text).
    /// `bg` : The background color (behind the text).
    /// `row` : The row which we want to set.
    /// `col` : The column which we want to set.
    #[inline]
    unsafe fn set_cell(&mut self, character: u8, fg: Color, bg: Color, row: usize, col: usize) {    
        // Calculate and set the cell based on the baground, foreground, and character.
        let char_colored: u16 = ((bg as u16) << 12) | ((fg as u16) << 8) | ((character as u16));
        
        // Calculate the cell pointer to set the character, and set it's value.
        *(self.get_cell_ptr(row, col) as *mut u16) = char_colored;
    }
    
    /// A method which clears a given cell at a row and column (sets it to empty space).
    ///
    /// # Parameters
    /// `row` : The row which we want to clear.
    /// `col` : The column which we want to clear.
    #[inline]
    unsafe fn clear_cell(&mut self, row: usize, col: usize) {
        // Set every cell to empty spaces.
        self.set_cell(0, Color::Black, Color::Black, row, col);
    }
    
    /// A method which copies the full cell data from the src to the dst cell. It includes the 
    /// character data in addition to the color data.
    ///
    /// # Parameters
    /// `src_row` : The row for the cell which we want to copy from.
    /// `src_col` : The column for the cell which we want to copy from.
    /// `dst_row` : The row for the cell which we want to copy to.
    /// `dst_col` : The column for the cell which we want to copy to.
    #[inline]
    unsafe fn copy_cell(&mut self, src_row: usize, src_col: usize, dst_row: usize, dst_col: usize) {
        // Copy the 16-bit value from the source to the destination cell;
        *(self.get_cell_ptr(dst_row, dst_col) as *mut u16) = 
            *(self.get_cell_ptr(src_row, src_col) as *mut u16);
    }
    
    /// A method which returns a character in a cell at a row and column. 
    ///
    /// # Parameters
    /// `row` : The row for the cell which we want to get.
    /// `col` : The column for the cell which we want to get.
    ///
    /// # Returns
    /// The 8_bit character value (ASCII) stored in the cell which was requested.
    #[inline]
    unsafe fn get_char(&mut self, row: usize, col: usize) -> u8 {    
        // Get the specified character byte from the given cell.
        *(self.get_cell_ptr(row, col))
    }
    
    /// A method which returns the color of foreground at a given row and column. 
    ///
    /// # Parameters
    /// `row` : The row for the cell which we want to get.
    /// `col` : The column for the cell which we want to get.
    ///
    /// # Returns
    /// The color enum which is used for the foreground.
    #[inline]
    unsafe fn get_fg(&mut self, row: usize, col: usize) -> Color {    
        // Get the specified color byte from the given cell.
        let color_byte: u8 = *((VGA_BUFFER + ((row * self.cols + col) * 2) + 1) as *mut u8);
        
        // Calculate the required Color enum and return it.
        Color::from(color_byte)
    }
    
    /// A method which returns the color of background at a given row and column. 
    ///
    /// # Parameters
    /// `row` : The row for the cell which we want to get.
    /// `col` : The column for the cell which we want to get.
    ///
    /// # Returns
    /// The color enum which is used for the background.
    #[inline]
    unsafe fn get_bg(&mut self, row: usize, col: usize) -> Color {    
        // Get the specified color byte from the given cell (and shift it by a byte).
        let color_byte: u8 = *((VGA_BUFFER + ((row * self.cols + col) * 2) + 1) as *mut u8) >> 8;
        
        // Calculate the required Color enum and return it.
        Color::from(color_byte)
    }
}

impl TextMode {
    /// A method which returns a pointer to a given cell. The pointer can then be cast for other
    /// types. It is used for conversion between rows and columns, to memory locations.
    #[inline]
    unsafe fn get_cell_ptr(&mut self, row: usize, col: usize) -> *mut u8 {
        (VGA_BUFFER + ((row * self.cols + col) * 2)) as *mut u8
    }
}
