//! A sub-module which implements the main round-robin scheduler and task switching. This
//! the scheduling sub-module in the architecture dependent code.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

#![allow(dead_code)]

use crate::arch::proc::process::scheduling;
use crate::proc::process::*;

/// Holds the current process which is linked to the rest of processes.
pub static mut PROC: *mut PCB = core::ptr::null_mut();

/// Process ID used for the IDLE process.
const IDLE_PID: usize = 0;

/// Holds the PID which will be assigned to the next task.
static mut CURR_PID: usize = 1;

/// Holds how many "ticks" each task runs for.
const MAX_TICKS: usize = 10;

/// Holds the current tick.
static mut CURR_TICK: usize = 0;

/// The high level scheduling algorithm which is called by the architecture 
/// dependent code to schedule the next task. It checks if it's time to context 
/// switch, and if it is, it gets the context of the next task, and replaces 
/// the current task in the interrupt stack.
///
/// # Parameters
/// `context` : The currently saved context which was saved by the interrupt handler.
pub unsafe fn schedule(context: *mut u8) {
    // Check if we're currently on the IDLE process.
    if (*PROC).pid == IDLE_PID {
        // If there are no more processes, simply return.
        if (*PROC).next == PROC {
            return;
        // Otherwise, set the tick to max to schedule the next process.
        } else {
            CURR_TICK = MAX_TICKS;
        }
    }

    // Check if the current task has finished it's time-slice.
    if CURR_TICK < MAX_TICKS {
        CURR_TICK += 1;
        return;
    } else {
        CURR_TICK = 0;
    }
    
    // Check it's current status.
    match (*PROC).status {
        // If it has already started.
        ProcessStatus::Started => {
            // Store the CPU context in the previous process (since it's done for now).
            crate::olibc::memcpy::memcpy((*PROC).context, context, CONTEXT_SIZE);
            
            // Go to the next process.
            PROC = (*PROC).next;
            
            // Set the context of CPU to the current context.
            scheduling::set_context(context, (*PROC).context);
        },
        
        // If the process has finished execution, remove it and shedule the next process.
        ProcessStatus::Exited => { 
            oxid_log!("Removed process PID={} from the scheduler.", (*PROC).pid);
            
            crate::olibc::memcpy::memcpy((*PROC).context, context, CONTEXT_SIZE);
            
            // Set the previous and next node pointers correctly.
            (*(*PROC).prev).next = (*PROC).next;
            (*(*PROC).next).prev = (*PROC).prev;
            
            // Store the pointer to the next.
            let next: *mut PCB = (*PROC).next;
            
            // Free the current exited PCB.
            PCB::free(PROC);
            
            // Store the next in line to schedule it.
            PROC = next;
            
            // Set the context of CPU to the current context.
            scheduling::set_context(context, (*PROC).context);
            
            // Set the current to none, and schedule the next.
            schedule(context);
        },
    }
    
   
}


/// A function which spawns a new process with a certain starting point, and name. 
/// It creates a new process control blocks, and adds it at the end of scheduled
/// processes.
///
/// # Parameters
/// `starting_ponit`: The function which will be called when executing.
/// `args` : The command line arguments passed.
/// `proc_name` : The name of the process.
pub unsafe fn spawn(starting_point: extern "sysv64" fn(*const Args), args: *mut Args
    , proc_name: &str) {
    oxid_log!("Spawning a new process. PID={}", CURR_PID);
    
    // Create a new PCB and put it at the end of the linked list.
    let new_pcb: *mut PCB = PCB::alloc(CURR_PID, proc_name, 
        (*PROC).prev, PROC);
        
    // Copy the arguments to it.
    (*new_pcb).args = *args;
        
    // Add the PCB at the end of list right before the current process.
    (*(*PROC).prev).next = new_pcb;
    (*PROC).prev = new_pcb;
    
    // Calculate the pointer stack start address (high-address).
    let stack_start = (((*new_pcb).stack_end as usize) + STACK_SIZE) as *mut u8;
    
    // Initialize the stack and starting point.
    scheduling::init_context(starting_point, exit, stack_start, 
        (*new_pcb).context, &(*new_pcb).args);
    
    // Increase the PID for the new process.
    CURR_PID += 1;
}

/// A function which initializes the scheduler by creating an adle process idle process.
/// and storing it.
pub unsafe fn init() {
    // Create an idle process and make it self referencing (for now).    
    PROC = PCB::alloc(IDLE_PID, "IDLE", 0x0 as *mut PCB, 0x0 as *mut PCB);
    
    // Reference itself.
    (*PROC).prev = PROC;
    (*PROC).next = PROC;
    
    // Calculate the pointer stack start address (high-address).
    let idle_stack_start = (((*PROC).stack_end as usize) + STACK_SIZE) as *mut u8;
    
    // Initialize the stack and starting point.
    scheduling::init_context(idle, exit, idle_stack_start, 
        (*PROC).context, &(*PROC).args);
}

/// A function which sets the status of the currently running process to exited. This should be 
/// called at the end of a processes execution timeline. However it is automatically called, so 
/// no explicit call is needed.
pub fn exit() {
    oxid_log!("Exiting Process.");
    unsafe {
        // If we're not in the IDLE process, set the status to exited.
        if (*PROC).pid != IDLE_PID {
            (*PROC).status = ProcessStatus::Exited;
        }
        
        // TODO: Better handling of EOI for when process hangs. 
        crate::arch::io::end_of_interrupt(); 
        
        // Put the system in low power mode for now.
        crate::arch::proc::halt();
    }
}

/// A function which kills the currently running process.
pub fn kill() {
    unsafe {
        // If we're not in the IDLE process, set the status to exited.
        if (*PROC).pid != IDLE_PID {
            oxid_warn!("Killing Process PID={}", (*PROC).pid);
            exit();
        } else {
            oxid_err!("No process to kill.")
        }
    }
}

/// A function which puts the system into low power mode for ever (idle).
///
/// `_args` : The arguments passed
pub extern "sysv64" fn idle(_args: *const Args) {
    unsafe { loop { crate::arch::proc::halt() }};
}
