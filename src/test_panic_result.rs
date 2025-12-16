//! Provider of [`TestPanicResult`].

use crate::*;
use std::any::Any;

/// Result of [`test_panic`](crate::test_panic) function.
#[must_use]
#[derive(Debug)]
pub enum TestPanicResult<R> {
    /// No panic result. Contains callback function result.
    Cool(R),
    /// Panic result. Contains panic payload.
    Panic(Box<dyn Any + Send>),
}

impl<R> TestPanicResult<R> {
    /// Returns `true` if self is [`Cool`](Self::Cool).
    #[must_use]
    pub fn is_cool(&self) -> bool {
        matches!(*self, Self::Cool(_))
    }

    /// Returns `true` if self is [`Panic`](Self::Panic).
    #[must_use]
    pub fn is_panic(&self) -> bool {
        matches!(*self, Self::Panic(_))
    }

    /// Converts `self` into success result if any.
    pub fn cool(self) -> Option<R> {
        match self {
            Self::Cool(x) => Some(x),
            Self::Panic(_) => None,
        }
    }

    /// Converts `self` into panic payload if any.
    pub fn panic(self) -> Option<Box<dyn Any + Send>> {
        match self {
            Self::Cool(_) => None,
            Self::Panic(x) => Some(x),
        }
    }

    /// Returns result value.
    ///
    /// # Panics
    ///
    /// Panics if self is [`Panic`](Self::Panic).
    #[must_use]
    pub fn value(&self) -> &R {
        match self {
            Self::Cool(x) => x,
            Self::Panic(_) => panic!("`self` is panic."),
        }
    }

    /// Returns panic payload.
    ///
    /// # Panics
    ///
    /// Panics if self is [`Cool`](Self::Cool).
    #[must_use]
    pub fn payload(&self) -> &Box<dyn Any + Send> {
        match self {
            Self::Cool(_) => panic!("`self` is cool."),
            Self::Panic(x) => x,
        }
    }

    /// Returns panic message.
    ///
    /// # Panics
    ///
    /// Panics if self is [`Cool`](Self::Cool) or panic payload is not
    /// [`&str`] or [`String`].
    #[must_use]
    pub fn message(&self) -> String {
        if self.is_cool() {
            panic!("`self` is cool.");
        }

        match self.get_message() {
            None => panic!("Panic payload is not string like."),
            Some(x) => x,
        }
    }

    /// Returns panic message if exists.
    #[must_use]
    pub fn get_message(&self) -> Option<String> {
        if self.is_cool() {
            return None;
        }

        util::string_like_to_string(self.payload().as_ref())
    }

    /// Tests `self` and `other` values to be nearly equal.
    ///
    /// This method ignores error payload. This is useful when comparing
    /// two functions with similar behavior, but where only the error
    /// messages differ.
    #[must_use]
    pub fn nearly_eq(&self, other: &Self) -> bool
    where
        R: PartialEq,
    {
        match (self, other) {
            (Self::Cool(l0), Self::Cool(r0)) => l0 == r0,
            (Self::Panic(_), Self::Panic(_)) => true,
            _ => false,
        }
    }
}

impl<R> Eq for TestPanicResult<R>
where
    R: Eq,
{
    // nop.
}

/// Compare result and error message.
///
/// # Notes
///
/// Errors are compared by [`Self::get_message`].
/// So, precise values of error payloads are not used.
impl<R> PartialEq for TestPanicResult<R>
where
    R: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Cool(l0), Self::Cool(r0)) => l0 == r0,
            (Self::Panic(_), Self::Panic(_)) => self.get_message() == other.get_message(),
            _ => false,
        }
    }
}
