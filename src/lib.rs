/*! Utility for test cases with panic.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

For the same purpose, the `shoud_panic` attribute is provided in the Rust
standard, but it is somewhat inflexible, hence we created this crate.

# Examples
```
#[test]
fn test() {
    let result = test_panic(|| panic!("message."));

    assert!(result.is_panic());
    assert_eq!(result.message(), "message");
}
```
*/

use std::any::Any;
use std::panic;
use std::panic::{AssertUnwindSafe, PanicInfo};

/// Execute the closure and get the panic result.
///
/// The default behavior in the event of a panic causes the standard error
/// output to show stack trace information, but since there is no use seeing
/// information about assumed errors, these behaviors are suppressed during
/// the execution of closure by this function.
///
/// # Notes
///
/// ###### Test only
///
/// This function is intended only for use in test automation and is not
/// suitable for other situations.
///
/// ###### After closure aborted
///
/// When panic occurs, closure execution is of course aborted. Especially in
/// functions called from closures, if such an abort is not assumed,
/// a logically inconsistent situation may occur. Therefore, access to each
/// state after a panic should be done with great care.
pub fn test_panic<F>(f: F) -> TestPanicResult
where
    F: FnOnce(),
{
    let default_panic_hook = panic::take_hook();
    let empty_panic_hook = Box::new(|_: &PanicInfo| {});

    panic::set_hook(empty_panic_hook);
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(default_panic_hook);

    if result.is_err() {
        return TestPanicResult::Panic(result.err().unwrap());
    }

    TestPanicResult::Cool
}

/// Result of [`test_panic`] method.
#[derive(Debug)]
pub enum TestPanicResult {
    /// No panic.
    Cool,
    /// Panic with some payload.
    Panic(Box<dyn Any + Send>),
}

impl TestPanicResult {
    /// Return `true` if self is [`Cool`](Self::Cool).
    pub fn is_cool(&self) -> bool {
        matches!(*self, Self::Cool)
    }

    /// Return `true` if self is [`Panic`](Self::Panic).
    pub fn is_panic(&self) -> bool {
        matches!(*self, Self::Panic(_))
    }

    /// Return panic payload.
    ///
    /// # Panics
    ///
    /// Panics if self is [`Cool`](Self::Cool).
    pub fn payload(&self) -> &Box<dyn Any + Send> {
        match self {
            Self::Cool => panic!("`self` is cool."),
            Self::Panic(x) => x,
        }
    }

    /// Return panic payload.
    ///
    /// # Panics
    ///
    /// Panics if self is [`Cool`](Self::Cool) or panic payload is not
    /// [`&str`] or [`String`].
    pub fn message(&self) -> String {
        if self.is_cool() {
            panic!("`self` is cool.");
        }

        match Self::string_like_to_string(self.payload().as_ref()) {
            None => panic!("Panic payload is not string like."),
            Some(x) => x,
        }
    }

    /// Converts string like value to string.
    fn string_like_to_string(any: &(dyn Any + Send)) -> Option<String> {
        if let Some(x) = any.downcast_ref::<&str>() {
            return Some(x.to_string());
        }

        if let Some(x) = any.downcast_ref::<String>() {
            return Some(x.to_owned());
        }

        None
    }
}
