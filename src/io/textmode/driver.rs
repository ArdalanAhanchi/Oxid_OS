//! A basic driver for the VGA text mode provided by the architecture. It allows writing bytes
//! at specified positions on the screen with a certain background and foreground colors.
//! this is the interface defenition, so the structs implementing this driver could serve for
//! textmode output.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

use crate::io::textmode::color::Color;

/// A group of functions which are needed for a text mode driver to be implemented.
pub trait Driver {
    /// Primary constructor which creates a vga buffer with a given row and column.
    /// it simply sets those values and returns it.
    ///
    /// # Parameters
    /// `rows` : The number of rows in the vga buffer.
    /// `cols` : The number of columns in the vga buffer.
    ///
    /// # Returns
    /// The newly created vga buffer.
    fn new(rows: usize, cols: usize) -> Self;

    /// Default constructor which creates a default vga buffer with the default number of rows.
    /// and columns for the architecture.
    ///
    /// # Returns
    /// The newly created vga buffer.
    fn new_default() -> Self;

    /// A public getter for the number of rows in the buffer.
    ///
    /// # Returns
    /// The number of rows in this object.
    fn get_rows(&self) -> usize;

    /// A public getter for the number of cols in the buffer.
    ///
    /// # Returns
    /// The number of cols in this object.
    fn get_cols(&self) -> usize;

    /// A method which sets a character in a cell at a row and column.
    ///
    /// # Parameters
    /// `character` : The ASCII byte which we want to set.
    /// `fg` : The foreground color (text).
    /// `bg` : The background color (behind the text).
    /// `row` : The row which we want to set.
    /// `col` : The column which we want to set.
    unsafe fn set_cell(&mut self, character: u8, fg: Color, bg: Color, row: usize, col: usize);

    /// A method which clears a given cell at a row and column (sets it to empty space).
    ///
    /// # Parameters
    /// `row` : The row which we want to clear.
    /// `col` : The column which we want to clear.
    unsafe fn clear_cell(&mut self, row: usize, col: usize);
    
    /// A method which copies the full cell data from the src to the dst cell. It includes the 
    /// character data in addition to the color data.
    ///
    /// # Parameters
    /// `src_row` : The row for the cell which we want to copy from.
    /// `src_col` : The column for the cell which we want to copy from.
    /// `dst_row` : The row for the cell which we want to copy to.
    /// `dst_col` : The column for the cell which we want to copy to.
    unsafe fn copy_cell(&mut self, src_row: usize, src_col: usize, dst_row: usize, dst_col: usize);

    /// A method which returns a character in a cell at a row and column.
    ///
    /// # Parameters
    /// `row` : The row for the cell which we want to get.
    /// `col` : The column for the cell which we want to get.
    ///
    /// # Returns
    /// The 8_bit character value (ASCII) stored in the cell which was requested.
    unsafe fn get_char(&mut self, row: usize, col: usize) -> u8;

    /// A method which returns the color of foreground at a given row and column.
    ///
    /// # Parameters
    /// `row` : The row for the cell which we want to get.
    /// `col` : The column for the cell which we want to get.
    ///
    /// # Returns
    /// The color enum which is used for the foreground.
    unsafe fn get_fg(&mut self, row: usize, col: usize) -> Color;

    /// A method which returns the color of background at a given row and column.
    ///
    /// # Parameters
    /// `row` : The row for the cell which we want to get.
    /// `col` : The column for the cell which we want to get.
    ///
    /// # Returns
    /// The color enum which is used for the background.
    unsafe fn get_bg(&mut self, row: usize, col: usize) -> Color;
}
