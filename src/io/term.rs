//! A basic temporary terminal implementation (without using files, they don't exist yet). It is 
//! currently directly called by the keyboard code. This module should be replaced in the future.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Mar 2021

#![allow(dead_code)]

use alloc::vec::Vec;                        // For managing strings.
use alloc::string::String;
use crate::io::keyboard::Key;               // For finding what key was pressed.
use crate::io::textmode::color::Color;      // For setting color (mainly for prompt).
use crate::proc::process::Args;

/// The buffer used for the terminal (will be cleared when user presses enter).
static mut TERM_BUFFER: Vec<char> = Vec::new();

/// The color of the prompt which will be printed on every line.
pub const PROMPT_COLOR: Color = Color::Cyan;

/// Hold the last parsed command and it's arguments.
pub static mut LAST_CMD_ARGS: Vec<&str> = Vec::new();

/// A function which initializes the terminal buffer. It should be called after the kernel heap
/// is already set up and working.
pub fn init() {
    // Print a terminal prompt.
    print_prompt();
}

/// A function which recieves a keypress from the keyboard driver, and acts accordingly.
///
/// # Parameters
/// `pressed` : The key which was pressed (can be of any type).
pub fn key_press(pressed: &Key) {
    unsafe {
        // Check which key was pressed and act accordingly.
        match pressed {
            // If it's just a character, add it to the buffer, and update the terminal.
            Key::Ch(character) => {
                // Add the character to the buffer.
                TERM_BUFFER.push(*character);
                
                // Then just print the line for now.
                oxid_print!("{}", character);
            },
            
            // If it's enter, process the buffer, clear it and go to the next line.
            Key::Enter => {
                process_buffer();
                TERM_BUFFER.clear();
                oxid_println!("");
                print_prompt();
            },
            
            // If it's backspace, pop the character from the buffer, and if the buffer is not empty
            // remove the last buffer from the screen.
            Key::Backspace => {
                if TERM_BUFFER.len() > 0 {
                    TERM_BUFFER.pop();
                    crate::console::CONSOLE.as_mut()
                        .expect("Console not initialized").clear_last_cell();
                }
            },
            
            // If it's escape, terminate the currently running program.
            Key::Esc => {
                crate::proc::scheduler::kill();
            }
            
            _ => {},
        }
    }
}

/// A function which processes the current buffer, and performs the appropriate tasks. For now some 
/// commands are hard coded, but in the future, it will call exec.
fn process_buffer() {
    unsafe {
        // Turn the commands into a string.
        let cmd_args_str: String = (&TERM_BUFFER).into_iter().collect();
        
        // Get the list of & seperated items.
        let whole_cmds: Vec<&str> = cmd_args_str.split("&").collect();
        let run_in_bg: bool = whole_cmds.len() > 1;
        
        // Go through each one of them.
        for cmd_arg in whole_cmds {
            // If there is only a single space and no command, don't execute.
            if cmd_arg.len() < 1 {
                continue;
            }
        
            // Split it by space.
            let cmds: Vec<&str> = cmd_arg.trim().split(" ").collect();
        
            // Check if the program exists, if it does, check if we're supposed to run in background.
            match crate::demo::get_main(cmds[0]) {
                Some(program_main) => {
                    // Allocate some memory for the arguments.
                    let args_ptr = crate::mem::dyn_alloc::kmalloc(core::mem::size_of::<Args>()
                        , false, true, false) as *mut Args;
                
                    // Set the arguments based on the passed data.
                    (*args_ptr).set_args(&cmd_arg.trim());
                    
                    // Check if we're supposed to run in background.
                    if run_in_bg {
                        // Spawn a new process.
                        crate::proc::scheduler::spawn(program_main, args_ptr, cmds[0]);
                    } else {
                        // Just run the program.
                        program_main(args_ptr);
                        
                        // Then deallocate the allocated memory.
                        crate::mem::dyn_alloc::kfree(args_ptr as *mut u8);
                    }
                },
                
                None => {
                    oxid_println!("");
                    oxid_err!("Could not find the {} command.", cmds[0]);
                }
            }
        }
        
    }
}

/// A simple function which prints the terminal prompt. Preferably, it does it in a different color.
#[inline]
fn print_prompt() {
    // Call the appropriate macro, don't 
    oxid_print_colored_nl!(crate::io::term::PROMPT_COLOR, crate::console::BG_COLOR, false, "Oxid > ");
}
