#![feature(core_intrinsics)]
#![feature(specialization)]

use std::fmt;
use std::fmt::Write;

use std::intrinsics::type_name;

/// Print a message, and then each value's debug representation (if it has one)
#[macro_export]
macro_rules! printdebug {
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

impl<T> fmt::Debug for DebugIt<T> {
    default fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            write!(f, "`{}`", type_name::<T>())
        }
    }
}

impl<T> fmt::Debug for DebugIt<T>
    where T: fmt::Debug
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn count_items<I>(iter: I) -> usize where I: IntoIterator {
    let mut c = 0;
    for elt in iter {
        printdebug!("Counting", elt);
        c += 1;
    }
    c
}

pub struct NoDebug<T>(pub T);

fn main() {
    let n = count_items(0..10);
    let n = count_items((0..10).map(|x| NoDebug(x)));
}
