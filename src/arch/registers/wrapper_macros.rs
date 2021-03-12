//! A sub-module which defines macros to wrap the getter and setters defined in the mod.asm
//! file. This allows rust access to setting or getting the register values.
//! 
//! `Author` : Ardalan Ahanchi
//! `Date` : Feb 2021.

#![allow(unused_macros)]

/// A macro that creates a wrapper for the getter with the given name (as the first parameter).
/// The name given should match the actual getter defined in the assembly file.
macro_rules! wrap_getter {
    ($getter_name:ident, $value_type:ty) => {
        extern "sysv64" {
            /// A standard getter which simply returns the value of the register.
            ///
            /// # Returns
            /// The current value stored in the register.
            pub fn $getter_name() -> $value_type;
        }
    }
}

/// A macro that creates a wrapper for the setter with the given name (as the first parameter).
/// The name given should match the actual setter defined in the assembly file.
macro_rules! wrap_setter {
    ($setter_name:ident, $value_type:ty) => {
        extern "sysv64" {
            /// A standard setter which simply sets the register to the given value.
            ///
            /// # Parameters
            /// `new_value` : The value which will be stored at the register.
            pub fn $setter_name(new_value: $value_type);
        }
    }
}

/// A macro that implement a getter and setter with the given names (as parameters). The first
/// parameter is the getter name, and the second is the setter name. 
/// The names given should match the actual getter and setter defined in the assembly file.
macro_rules! wrap_accessors {
    ($getter_name:ident, $setter_name:ident, $value_type:ty) => {
        wrap_getter!($getter_name, $value_type);
        wrap_setter!($setter_name, $value_type);
    }
}
