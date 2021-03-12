//! A sub-module which implements the main round-robin scheduler and task switching. This
//! the scheduling sub-module in the architecture dependent code.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

#![allow(dead_code)]

use alloc::string::String;
use alloc::collections::LinkedList;
use alloc::boxed::Box;
use crate::arch::proc::process::scheduling;
use crate::proc::process::{ProcessStatus, PCB, Args};

/// Holds the linked list for the round-robin scheduler.
pub static mut PROCESSES: LinkedList<Box<PCB>> = LinkedList::new();

/// Holds the currently executing process.
static mut CURR_PROCESS: Option<*mut PCB> = None;

/// Hold a process for an idle task which will be context switched if there is nothing to run.
static mut IDLE_PROCESS: Option<PCB> = None;

/// Holds the PID which will be assigned to the next task.
static mut CURR_PID: usize = 1;

/// Holds the size of the stack which will be allocated.
const STACK_SIZE: usize = 0x1000;

/// Holds the size of the context (from architecture dependent code).
const CONTEXT_SIZE: usize = scheduling::context_size();

/// Holds how many "ticks" each task runs for.
const MAX_TICKS: usize = 5;

/// Holds the current tick.
static mut CURR_TICK: usize = 0;

/// The high level scheduling algorithm which is called by the architecture dependent code to 
/// schedule the next task. It checks if it's time to context switch, and if it is, it gets the
/// context of the next task, and replaces the current task in the interrupt stack.
///
/// # Parameters
/// `context` : The currently saved context which was saved by the interrupt handler.
pub unsafe fn schedule(context: *mut u8) {
    // Check if the current task has finished it's time-slice.
    if CURR_TICK < MAX_TICKS {
        CURR_TICK += 1;
        return;
    } else {
        CURR_TICK = 0;
    }

    // If there are any processes to schedule, schedule them.
    if PROCESSES.len() != 0 {
        // Get the currently running process from the start of the list.
        let mut curr = PROCESSES.pop_front().expect("Could not unwrap PCB.");
        
        // Check it's current status.
        match curr.status {
            // If it has already started.
            ProcessStatus::Started => {
                // If there is a currently running process, save it's context.
                if CURR_PROCESS.is_some() {
                    // Store the CPU context in the previous process (since it's done for now).
                    crate::olibc::memcpy::memcpy((*CURR_PROCESS.unwrap()).context
                        , context, CONTEXT_SIZE);
                }
            
                // Set the context of CPU to the current context.
                scheduling::set_context(context, curr.context);
                
                // Set the current process, and then add it back to the linked list.
                CURR_PROCESS = Some(curr.as_mut() as *mut PCB);
                PROCESSES.push_back(curr);
            },
            
            // If the process has finished execution, remove it and shedule the next process.
            ProcessStatus::Exited => { 
                oxid_println!();
                oxid_log!("Removed process PID={} from the scheduler.", (*curr).pid);
                
                // Deallocate the stack and context which were allocated in spawn.
                //crate::mem::dyn_alloc::kfree((*curr).stack_end);
                crate::mem::dyn_alloc::kfree((*curr).context);
                
                // If arguments were allocated, deallocte them.
                if curr.args != 0 as *mut Args {
                     crate::mem::dyn_alloc::kfree((*curr).args as *mut u8);
                }
                
                // Set the current to none, and schedule the next.
                CURR_PROCESS = None; 
                schedule(context);
            },
        }
    } else {
        scheduling::set_context(context, IDLE_PROCESS.as_mut().unwrap().context);
    }
}


/// A function which spawns a new process with a certain starting point, and name. It simply creates 
/// a new process control blocks, and sets it in the list of processes.
///
/// # Parameters
/// `starting_ponit`: The function which will be called when executing.
/// `args` : The command line arguments passed.
/// `proc_name`: The name of the process.
pub unsafe fn spawn(starting_point: extern "sysv64" fn(*const Args), args: *mut Args
    , proc_name: &str) {
    oxid_log!("Spawning a new process. PID={}", CURR_PID);

    // If no memory is allocated for context, allocate some.
    let new_pcb = PCB {
        pid: CURR_PID,
        name: String::from(proc_name),
        status: ProcessStatus::Started,
        stack_end: crate::mem::dyn_alloc::kmalloc(STACK_SIZE, false, true, false),
        context: crate::mem::dyn_alloc::kmalloc(CONTEXT_SIZE, false, true, false),
        args: args,
    };
    
    // Calculate the pointer stack start address (high-address).
    let stack_start = ((new_pcb.stack_end as usize) + STACK_SIZE) as *mut u8;
    
    // Initialize the stack and starting point.
    scheduling::init_context(starting_point, exit, stack_start, new_pcb.context, new_pcb.args);
    
    // Increase the PID and add the process to the list.
    CURR_PID += 1;
    PROCESSES.push_back(Box::new(new_pcb));
}

/// A function which initializes the scheduler by creating an adle process idle process.
/// and storing it.
pub unsafe fn init() {
    // Create an idle process.
    let idle_pcb = PCB {
        pid: 0,
        name: String::from("IDLE"),
        status: ProcessStatus::Started,
        stack_end: crate::mem::dyn_alloc::kmalloc(STACK_SIZE, false, true, false),
        context: crate::mem::dyn_alloc::kmalloc(CONTEXT_SIZE, false, true, false),  
        args: &mut Args::new(),  
    };
    
    // Calculate the pointer stack start address (high-address).
    let idle_stack_start = ((idle_pcb.stack_end as usize) + STACK_SIZE) as *mut u8;
    
    // Initialize the stack and starting point.
    scheduling::init_context(idle, exit, idle_stack_start, idle_pcb.context, idle_pcb.args);
    
    // Calculate the pointer stack start address (high-address).
    IDLE_PROCESS = Some(idle_pcb);
}

/// A function which sets the status of the currently running process to exited. This should be 
/// called at the end of a processes execution timeline. However it is automatically called, so 
/// no explicit call is needed.
pub fn exit() {
    unsafe {
        // Get the currently running process.
        let proc = CURR_PROCESS.expect("Exit should be called on a running process.");
        
        // Set it's status to end.
        (*proc).status = ProcessStatus::Exited;
        
        // Make sure the current process is None.
        CURR_PROCESS = None;
    }
    
    // Put the system in low power mode for now.
    unsafe { crate::arch::proc::halt() };
}

/// A function which kills the currently running process.
pub fn kill() {
    unsafe {
        match CURR_PROCESS {
            Some(proc) => { 
                oxid_warn!("Killing Process PID={}", (*proc).pid);
                exit();
            },
            
            None => { oxid_err!("No process to kill.") },
        }
    }
}

/// A function which puts the system into low power mode for ever (idle).
///
/// `_args` : The arguments passed
pub extern "sysv64" fn idle(_args: *const Args) {
    unsafe { loop { crate::arch::proc::halt() }};
}
