//! A sub-module which provides the translation of keyboard scan codes, and the handling of  
//! keyboard events. This will be called by the keyboard driver when interrupts occur.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

#![allow(dead_code)]

pub mod ps2;

static mut SHIFT_PRESSED: bool = false;
static mut IS_CAPS: bool = false;
static mut IS_NUM_LOCK: bool = false;

/// An enum which represents a key. It can be of any of the following types. It is used for 
/// translation of the key codes and proper handling of them.
#[derive(Copy, Clone)]
pub enum Key {
    Ch(char),       // Represent a character.
    F(u8),          // Function keys (F1-F12).
    Esc,            // Escape key.
    LShift,         // Left shift.
    RShift,         // Right shift.
    LAlt,           // Left alt.
    RAlt,           // Right alt.
    LCtrl,          // Left control.
    RCtrl,          // Right control.
    CapsLock,       // Caps lock.
    NumLock,        // Num lock.
    ScrlLock,       // Scroll lock.
    Enter,          // Enter key.
    Backspace,      // Backspace key.
    Null,           // No key.
}

/// A structure which represents a keyboard event (key press, release, etc.).
pub struct Event {
    key: Key,               // The key which was translated.
    pressed: bool,          // True if key was pressed, False if released.
}

/// A function which is called by the keyboard drivers with a given event. It will try to handle
/// the event gracefully and handles the upper/lower case modifiers.
///
/// # Parameters
/// `event` : The keyboard event which occured (key and pressed information).
pub fn handle_event(event: &Event) {
    // Check if the key was pressed.
    if event.pressed {
        // For now, just print the event if it's a character.
        match event.key {
            // If it's a character, process and send it.
            Key::Ch(character) => send_key(Key::Ch(process_character(character))),
            
            // If it's shift is pressed simply set the variable.
            Key::LShift | Key::RShift => unsafe { SHIFT_PRESSED = true },
            
            // Toggle the caps.
            Key::CapsLock => unsafe { IS_CAPS = !IS_CAPS },
            
            // Otherwise, just send the key as it.
            _ => send_key(event.key),
        }
    } else {
        // If the key got released, check for shift and other modifiers.
        match event.key {
            // If it's released, clear the variable.
            Key::LShift | Key::RShift => unsafe { SHIFT_PRESSED = false },
            
            // Otherwise, no need to do anything.
            _ => (),
        }
    }
}


/// A function which hanled a given character and processes it if necessary (for example, if it is 
/// supposed to be caps, or the modifier keys are pressed).
///
/// # Parameters
/// `character` : The character from the key (ASCII).
#[inline]
fn process_character(character: char) -> char {
    unsafe {
        // To modify based on the modifiers.
        let mut transformed_char = character;
    
        // Check if shift is currently pressed, and print it accordingly.
        if SHIFT_PRESSED {
            transformed_char = match character {
                // If any of the modified characters, translate them.
                '1' => '!', '2' => '@', '3' => '#', '4' => '$', '5' => '%', '6' => '^', 
                '7' => '&', '8' => '*', '9' => '(', '0' => ')', '-' => '_', '=' => '+',
                '`' => '~', '[' => '{', ']' => '}', '\\' => '|', ';' => ':', '\'' => '"',
                ',' => '<', '.' => '>', '/' => '?',
                
                // Otherwise, just translate it to uppercase.
                _ => if IS_CAPS {
                        character
                    } else {
                        character.to_ascii_uppercase()
                    }
            };
        } else if IS_CAPS {
            transformed_char = character.to_ascii_uppercase()
        }
        
        // Return the transformed character.
        transformed_char
    }
}


/// A function which sends a given key press to the appropriate place. This will be replaced in the
/// future to send it to some sort of a file (or somewhere else that does that).
#[inline]
fn send_key(to_send: Key) {
    // Just send it to the terminal for now.
    crate::io::term::key_press(&to_send);
}
