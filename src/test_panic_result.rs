//! Provider of [`TestPanicResult`].

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
