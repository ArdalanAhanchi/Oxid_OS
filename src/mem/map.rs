//! File which defines the constants and statics which define the memory map for the kernel
//! address space.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : March 2021

/// The address where the kernel ends (also end of id mapped area, set by mem::mod.rs).
pub static mut KERNEL_END_ADDR: usize = 0;

/// The maximum address which can be used to store heap metadata (set it at the end of 4GB mark).
pub const KERNEL_HEAP_METADATA_END_ADDR: usize = 0x100000000;

/// The maximum address covered in the heap (Set it at the end of lower half).
pub const KERNEL_HEAP_END_ADDR: usize = 0x7FFFFFFFFFFF;

/// The starting address where the page tables are stored (set by arch::mem::page_tables).
pub const PAGE_TABLES_START_ADDR: usize = crate::arch::mem::page_tables::PAGE_TABLES_VM_START;

/// The ending address where the page tables are stored (set by arch::mem::page_tables).
pub const PAGE_TABLES_END_ADDR: usize = crate::arch::mem::page_tables::PAGE_TABLES_VM_END;
