//! Crate's messages.

use std::fmt::Debug;

/// Message for failed equal almost assertion.
pub fn eqa_failed<T>(lft: &T, rgt: &T) -> String
where
    T: Debug,
{
    format!(
        "Assertion left and right almost equal failed.\
        \n left: {lft:?}\
        \nright: {rgt:?}",
    )
}

/// Message for failed equal nearly assertion.
pub fn eqn_failed<T>(lft: &T, rgt: &T) -> String
where
    T: Debug,
{
    format!(
        "Assertion left and right nearly equal failed.\
        \n left: {lft:?}\
        \nright: {rgt:?}",
    )
}
