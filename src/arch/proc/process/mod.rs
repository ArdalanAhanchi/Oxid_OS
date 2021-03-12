//! A module which includes the process (task) management code for this architecture. 
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

#![allow(dead_code)]        // To allow these to exist without actually calling them.

mod tss;
pub mod scheduling;

static mut CURR_TSS: tss::TSS = tss::TSS::new();

pub unsafe fn init() {
    CURR_TSS.load(0);
}
