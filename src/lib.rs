#![cfg_attr(use_nightly, feature(core_intrinsics, specialization))]

//! Use debug printlns, without the trait bounds (using specialization to
//! find the right impl anyway).
//! 

use std::fmt;

#[cfg(use_nightly)]
use std::intrinsics::type_name;

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
///     debugit!("starting with", x);
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
/// Otherwise, falls back to print something else.
///
/// On Rust stable, it always has to print something else.
///
/// # Examples
///
/// ```
/// use debugit::DebugIt as D;
///
/// fn process_something<T>(x: T) {
///     println!("starting with {:?}", D(&x));
/// }
/// ```
#[derive(Copy, Clone)]
pub struct DebugIt<T>(pub T);

#[cfg(not(use_nightly))]
impl<T> fmt::Debug for DebugIt<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<unknown>")
    }
}

#[cfg(use_nightly)]
impl<T> fmt::Debug for DebugIt<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "`{}`", type_name::<T>())
        }
    }
}

#[cfg(use_nightly)]
impl<T> fmt::Debug for DebugIt<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use ::DebugIt;

    #[test]
    fn it_works() {
        fn debug_no_bound<T>(x: T, s: &str) {
            assert_eq!(&format!("{:?}", DebugIt(&x)), s);
        }
        debug_no_bound(1, "1");
        debug_no_bound((), "()");
    }
}
