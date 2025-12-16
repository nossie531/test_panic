//! Provider of [`test_panic`].

use crate::*;
use std::panic;
use std::panic::{AssertUnwindSafe, PanicHookInfo};

/// Execute the closure and get its return value or panic information.
///
/// In context of this function, even if a closure panics, writing to the
/// standard error output about that panic will not be performed. This is
/// because it is not useful to see information about errors as planned
/// for testing.
///
/// # Notes
///
/// ###### Test only
///
/// This function is intended only for use in test automation and is not
/// suitable for other situations.
///
/// ###### After panic
///
/// If `f` panics, the logical invariance condition can be broken unless
/// the operations in `f` take post-panic operations into account. Therefore,
/// access to each state after panic should be done with great care.
pub fn test_panic<F, R>(f: F) -> TestPanicResult<R>
where
    F: FnOnce() -> R,
{
    let default_panic_hook = panic::take_hook();
    let empty_panic_hook = Box::new(|_: &PanicHookInfo| {});

    panic::set_hook(empty_panic_hook);
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(default_panic_hook);

    if result.is_err() {
        return TestPanicResult::Panic(result.err().unwrap());
    }

    TestPanicResult::Cool(result.unwrap())
}
