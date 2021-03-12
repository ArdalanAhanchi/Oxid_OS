//! A module which includes a set of c library functions which are absolutely needed to run 
//! the core rust library. They are specifically needed by rustc, and core.
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

pub mod memcmp;
pub mod memcpy;
pub mod memset;
pub mod memmove;
