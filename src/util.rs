//! Crate's utility.

use std::any::Any;

/// Converts string like value to string.
pub(crate) fn string_like_to_string(any: &(dyn Any + Send)) -> Option<String> {
    if let Some(x) = any.downcast_ref::<&str>() {
        return Some(x.to_string());
    }

    if let Some(x) = any.downcast_ref::<String>() {
        return Some(x.to_owned());
    }

    None
}
