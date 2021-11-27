#![feature(specialization)]

//! Use debug printlns, without the trait bounds (using specialization to
//! find the right impl anyway).
//!
//! **NOTE**: This uses experimental Rust features and is therefore
//! by itself experimental and unstable, and has all the problems of
//! `feature(specialization)`.
//!
//! For this reason, `unsafe` is required to use this feature unfortunately.
//! 

use std::fmt;
use std::any::type_name;

/// Print a message, and then each value's debug representation (if it has one)
///
/// NOTE: This macro has **no** format string, only a message and a list of values.
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate debugit;
///
/// fn process_something<T>(x: T) {
///     unsafe {
///         debugit!("starting with", x);
///     }
/// }
///
/// # fn main() { }
/// ```
#[macro_export]
macro_rules! debugit {
    ($message:expr, $($value:expr),*) => {{
        print!("{} ", $message);
        $(
            print!("{:?}", $crate::DebugIt(&$value));
        )*
        println!("");
    }};
}

/// This type always implements `Debug`. Uses specialization to use
/// the inner value's Debug (which it should basically always have).
///
/// Otherwise, falls back to print the type name.
///
/// # Examples
///
/// ```
/// use debugit::DebugIt as D;
///
/// fn process_something<T>(x: T) {
///     unsafe {
///         println!("starting with {:?}", D(&x));
///     }
/// }
/// ```
#[derive(Copy, Clone)]
pub struct DebugIt<T> {
    value: T
}

/// This type always implements `Debug`. Uses specialization to use
/// the inner value's Debug (which it should basically always have).
///
/// Otherwise, falls back to print the type name.
///
/// # Examples
///
/// ```
/// use debugit::DebugIt as D;
///
/// fn process_something<T>(x: T) {
///     unsafe {
///         println!("starting with {:?}", D(&x));
///     }
/// }
/// ```
pub unsafe fn DebugIt<T>(value: T) -> DebugIt<T> {
    DebugIt { value }
}

impl<T> fmt::Debug for DebugIt<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", type_name::<T>())
    }
}

impl<T> fmt::Debug for DebugIt<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::DebugIt;

    #[test]
    fn it_works() {
        fn debug_no_bound<T>(x: T, s: &str) {
            unsafe {
                assert_eq!(&format!("{:?}", DebugIt(&x)), s);
            }
        }
        debug_no_bound(1, "1");
        debug_no_bound((), "()");
    }
}
