//! A console wrapper which provides a static console that everything can write to. It also
//! provides many macros for different types of formatted printing to the console such as print(ln), 
//! logging, warnining, and error messages. The allows various levels of messages to be printed 
//! to the writer (with appropriate colors and header messages).
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

// Some of the resources used:
// https://doc.rust-lang.org/rust-by-example/macros/syntax.html
// https://doc.rust-lang.org/src/std/macros.rs.html#92-97

#![allow(dead_code)]              // Because we might not use all functions or attributes.
#![allow(unused_macros)]
#![macro_use]

use crate::arch::io::textmode::TextMode;
use crate::io::textmode::{driver::Driver, writer::Writer, color::Color};

pub const BG_COLOR: Color = Color::Black;               // The background color used.
pub const TEXT_COLOR: Color = Color::Gray;              // The default text color.
pub const LOG_COLOR: Color = Color::Green;              // The color for the log messages.
pub const WARN_COLOR: Color = Color::Yellow;            // The color for the warning messages.
pub const ERR_COLOR: Color = Color::Red;                // The color for the error messages.

/// A static console which we can use to write globally.
// pub static mut CONSOLE: Option<Writer<TextMode>> = None;
pub static mut CONSOLE: Option<Writer<TextMode>> = None;

/// A function which initializes the console which is statically available. It allows global writes
/// to the console. It initializes the driver which is used, and a writer, and stores it in CONSOLE.
pub unsafe fn init() {
    let tm_driver_x86: TextMode = TextMode::new_default();              // Initialize the driver.
    let tm_writer: Writer<TextMode> = Writer::new(tm_driver_x86);       // Initialize the writer.
    CONSOLE = Some(tm_writer);                                          // Store the global console.
}

/// A macro which performs a regular print without needing a newline. It can accepts all kinds of 
/// inputs (as long as they are tt's). The pattern matching was inspired rom the rust std library's
/// implementation of print! macro. Which can be found at:
/// https://doc.rust-lang.org/src/std/macros.rs.html#92-97
macro_rules! oxid_print {
    ($($arg:tt)*) => ({
        oxid_print_colored_nl!(crate::console::TEXT_COLOR, crate::console::BG_COLOR, false, $($arg)*);
    });
}

/// A macro which performs a print and then adds a newline to it. It can accepts all kinds of 
/// inputs (as long as they are tt's). The pattern matching was inspired rom the rust std library's
/// implementation of println! macro. Which can be found at:
/// https://doc.rust-lang.org/src/std/macros.rs.html#92-97
macro_rules! oxid_println {
    // If no arguments were passed, call itself with a newline character.
    () => (oxid_print!("\n"));

    ($($arg:tt)*) => ({
        oxid_print_colored_nl!(crate::console::TEXT_COLOR, crate::console::BG_COLOR, true, $($arg)*);
    })
}

/// A macro which is similar to a println, but it is meant for regular log messages. The color is 
/// different from regular messages, and it adds a header to indicate what type of message it is. 
/// The pattern matching was inspired rom the rust std library's implementation of print! macro,
/// Which can be found at: https://doc.rust-lang.org/src/std/macros.rs.html#92-97
macro_rules! oxid_log {
    ($($arg:tt)*) => ({
        // Print the log header (without a newline), and then print the message.
        oxid_print_colored_nl!(crate::console::LOG_COLOR, crate::console::BG_COLOR, false, "Oxid: Log: ");
        oxid_print_colored_nl!(crate::console::LOG_COLOR, crate::console::BG_COLOR, true, $($arg)*);
    });
}

/// A macro which is similar to a println, but it is meant for warning messages. The color is 
/// different from regular messages, and it adds a header to indicate what type of message it is. 
/// The pattern matching was inspired rom the rust std library's implementation of print! macro,
/// Which can be found at: https://doc.rust-lang.org/src/std/macros.rs.html#92-97
macro_rules! oxid_warn {
    ($($arg:tt)*) => ({
        // Print the warn header (without a newline), and then print the message.
        oxid_print_colored_nl!(crate::console::WARN_COLOR, crate::console::BG_COLOR, false, "Oxid: Warn: ");
        oxid_print_colored_nl!(crate::console::WARN_COLOR, crate::console::BG_COLOR, true, $($arg)*);
    });
}

/// A macro which is similar to a println, but it is meant for error messages. The color is 
/// different from regular messages, and it adds a header to indicate what type of message it is. 
/// The pattern matching was inspired rom the rust std library's implementation of print! macro,
/// Which can be found at: https://doc.rust-lang.org/src/std/macros.rs.html#92-97
macro_rules! oxid_err {
    ($($arg:tt)*) => ({
        // Print the err header (without a newline), and then print the message.
        oxid_print_colored_nl!(crate::console::ERR_COLOR, crate::console::BG_COLOR, false, "Oxid: Err: ");
        oxid_print_colored_nl!(crate::console::ERR_COLOR, crate::console::BG_COLOR, true, $($arg)*);
    });
}

/// The main backbone behind all the implemented macros for formatted printing in oxid os. It allows
/// colored printing (specified foreground and backgroun colors), and allows adding a newline at the
/// end of the printing if requested. It is used to merge all the sensitive code into one macro.
///
/// # Required Parameters
/// `fg` : The color for the foreground (should be of type Color).
/// `bg` : The color for the background (should be of type Color).
/// `add_nl` : A boolean which should determine if we want to add a newline. 
macro_rules! oxid_print_colored_nl {
    // Accept a foreground color, backgroun color, boolean (to ask if we need a newline), and args.
    ($fg:expr, $bg:expr, $add_nl:expr, $($arg:tt)*) => ({
        #[allow(unused_unsafe)]                              // So we can use it in unsafe functions.
        unsafe {  
            match &mut crate::console::CONSOLE {             // Check if the console is initialized.
                Some(w) => {
                    #[allow(unused_imports)]
                    use core::fmt::Write;
                    
                    // Set the color to the passed text colors (in case something changed it).
                    w.set_colors($fg, $bg);
                    w.write_fmt(format_args!($($arg)*)).unwrap();           // Write fmt to console.
                    
                    if $add_nl {                              // If a newline was requested, add it.
                        w.print("\n");
                    }
                },

                None => panic!("Console not initialized"),
            }
        }
    })
}

