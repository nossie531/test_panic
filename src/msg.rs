//! Crate's messages.

use std::fmt::Debug;

/// Message for failed equal almost assertion.
pub fn eqa_failed<D1, D2>(lft: &D1, rgt: &D2) -> String
where
    D1: Debug,
    D2: Debug,
{
    format!(
        "Assertion left and right almost equal failed.\
        \n left: {lft:?}\
        \nright: {rgt:?}",
    )
}

/// Message for failed equal nearly assertion.
pub fn eqn_failed<D1, D2>(lft: &D1, rgt: &D2) -> String
where
    D1: Debug,
    D2: Debug,
{
    format!(
        "Assertion left and right nearly equal failed.\
        \n left: {lft:?}\
        \nright: {rgt:?}",
    )
}
