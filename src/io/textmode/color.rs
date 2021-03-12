//! Some definitions to represent the color codes of the VGA specification (4-bit). Additionally,
//! it includes the conversion from the raw nibble to the color enum.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

/// Represent the color codes used in the 4 bit color pallete used in the VGA
/// specifications. the order of these colors matches the standard specified.
/// https://wiki.osdev.org/Printing_to_Screen
#[repr(u8)]
#[derive(Eq, PartialEq, Copy, Clone)]          // To make sure we can pass it around and compare it.
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Brown,
    Gray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    LightPurple,
    Yellow,
    White,
}

/// Allow the conversion of unsigned byte sized integers to Color types.
impl From<u8> for Color {

    /// Allow the conversion of a nibble to the color. Only the first 4 bits of the "val" will be
    /// taken into account in the conversion. This way, it is guaranteed that there will always be
    /// a correct color which is returned.
    ///
    /// # Parameters
    /// `val` : The byte which contains the first nibble which represents the color.
    ///
    /// # Returns
    /// The corresponding color to the first nibble.
    fn from(val: u8) -> Self {
        // Filter the val to make sure it's first 4 bits are 0.
        let masked_val: u8 = val % 0x10;

        // Match it to the correct color.
        match masked_val {
            0x0 => Color::Black,
            0x1 => Color::Blue,
            0x2 => Color::Green,
            0x3 => Color::Cyan,
            0x4 => Color::Red,
            0x5 => Color::Purple,
            0x6 => Color::Brown,
            0x7 => Color::Gray,
            0x8 => Color::DarkGray,
            0x9 => Color::LightBlue,
            0xA => Color::LightGreen,
            0xB => Color::LightCyan,
            0xC => Color::LightRed,
            0xD => Color::LightPurple,
            0xE => Color::Yellow,
            0xF => Color::White,
            _ => Color::Black,
        }
    }
}
