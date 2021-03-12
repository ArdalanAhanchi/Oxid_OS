//! A sub-module which is the primary high level handler for x86_64 exceptions. All the exception
//! interrupts should be register_traped here (interrupts 0-31). Each exception should be handled in
//! their own seperate file.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021

#![allow(dead_code)]

mod divide_by_zero;
mod trap_flag;
mod non_maskable_int;
mod break_point;
mod overflow;
mod bound_range;
mod invalid_opcode;
mod device_not_available;
mod double_fault;
mod invalid_tss;
mod segment_not_present;
mod stack_fault;
mod general_protection_fault;
mod page_fault;
mod math_fault;
mod alignment_check;
mod machine_check;
mod simd_fp;

/// A function which initializes all the default handlers for the exceptions with their 
/// corresponding interrupt numbers as specified by the AMD64 programming manual.
/// More details can be found at: https://en.wikipedia.org/wiki/Interrupt_descriptor_table
pub fn init() {
    super::register_trap(0x0, divide_by_zero::handle);
    super::register_trap(0x1, trap_flag::handle);
    super::register_trap(0x2, non_maskable_int::handle);
    super::register_trap(0x3, break_point::handle);
    super::register_trap(0x4, overflow::handle);
    super::register_trap(0x5, bound_range::handle);
    super::register_trap(0x6, invalid_opcode::handle);
    super::register_trap(0x7, device_not_available::handle);
    super::register_trap(0x8, double_fault::handle);
    super::register_trap(0xa, invalid_tss::handle);
    super::register_trap(0xb, segment_not_present::handle);
    super::register_trap(0xc, stack_fault::handle);
    super::register_trap(0xd, general_protection_fault::handle);
    super::register_trap(0xe, page_fault::handle);
    super::register_trap(0x10, math_fault::handle);
    super::register_trap(0x11, alignment_check::handle);
    super::register_trap(0x12, machine_check::handle);
    super::register_trap(0x13, simd_fp::handle);
}
