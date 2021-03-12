//! A sub-module which provides the translation of PS2 scan codes. It also handles shift and other 
//! modifiers needed. 
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021
//! A sub-module which provides the translation of PS2 scan codes, this code is directly called when
//! an event occurs and it translates the code and calls the parent with the given key.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

#![allow(dead_code)]

/// A sub-module for translation of SetOne key codes into a key enum.
pub mod set_1 {
    // Bring the key and all it's element into scope.
    use crate::io::keyboard::{Key, Event};
    use crate::io::keyboard::Key::*;

    /// A translation table for the PS2 set 1 (US QWERTY) scan codes. This table is ordered and can
    /// be directly indexed into from idx 0 to 88 which includes all the key presses. For now, it 
    /// does not support extended or multimedia keys. More details about these codes can be
    /// found at: https://wiki.osdev.org/PS2_Keyboard
    const SCAN_CODES: [Key; 0x59] = [Null, Esc, Ch('1'), Ch('2'), Ch('3'),
        Ch('4'), Ch('5'), Ch('6'), Ch('7'), Ch('8'), Ch('9'), Ch('0'), Ch('-'),
        Ch('='), Backspace, Ch('\t'), Ch('q'), Ch('w'), Ch('e'), Ch('r'), Ch('t'),
        Ch('y'), Ch('u'), Ch('i'), Ch('o'), Ch('p'), Ch('['), Ch(']'), Enter, 
        LCtrl, Ch('a'), Ch('s'), Ch('d'), Ch('f'), Ch('g'), Ch('h'), Ch('j'), 
        Ch('k'), Ch('l'), Ch(';'), Ch('\''), Ch('`'), LShift, Ch('\\'), Ch('z'), 
        Ch('x'), Ch('c'), Ch('v'), Ch('b'), Ch('n'), Ch('m'), Ch(','), Ch('.'), 
        Ch('/'), RShift, Ch('*'), LAlt, Ch(' '), CapsLock, F(1), F(2), 
        F(3), F(4), F(5), F(6), F(7), F(8), F(9), F(10), NumLock, ScrlLock, 
        Ch('7'), Ch('8'), Ch('9'), Ch('-'), Ch('4'), Ch('5'), Ch('6'), Ch('+'), 
        Ch('1'), Ch('2'), Ch('3'), Ch('0'), Ch('.'), Null, Null, Null, F(11), F(12)];
        
    /// A function which can translate a given key_code to a key structure. It is typically used by
    /// the PS2 driver to get a Key and call an event in the Keyboard code.
    ///
    /// # Parameters
    /// `key_code`: The raw key code which was recieved.
    ///
    /// # Returns
    /// A keyboard event to be handled by the event handler.
    pub fn translate(key_code: u8) -> Event {
        // The following conditions are based on the table defined for the set 1. More information
        // can be found at https://wiki.osdev.org/PS2_Keyboard.
        if key_code < 0x59 {
            // In this case, the key was pressed (based on the Set 1 table).
            Event {
                key: SCAN_CODES[key_code as usize],
                pressed: true,
            }
        } else if key_code > 0x80 && key_code < 0xD9 {
            // In this case, the key was released (based on the Set 1 table).
            Event {
                key: SCAN_CODES[(key_code - 0x80) as usize],
                pressed: false,
            }
        } else {
            // These codes are not supported yet, so return Null.
            Event {
                key: Null,
                pressed: true,
            }
        }
    }

}
