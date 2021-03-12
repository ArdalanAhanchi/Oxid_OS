//! A module which implements the basic VESA textmode driver (abstracted, and independent of 
//! hardware). It also defines a basic driver which can be implemented based on the architecture
//! to allow access to it. The code here should be re-usable within various VESA modes. 
//!
//! `Author` : Ardalan Ahanchi
//! `Date` : Jan 2021

pub mod driver;
pub mod color;
pub mod writer;
