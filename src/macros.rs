//! Crate's macros.

/// Asserts that two expressions are almost equal (using [`eq_almost`]).
///
/// [`eq_almost`]: crate::TestPanicResult::eq_almost
#[macro_export]
macro_rules! assert_eqa {
    ($lft:expr, $rgt:expr $(,)?) => {
        if !$crate::TestPanicResult::eq_almost(&$lft, &$rgt) {
            panic!("{}", $crate::msg::eqa_failed(&$lft, &$rgt));
        }
    };

    ($lft:expr, $rgt:expr, $($arg:tt)+) => {
        if !$crate::TestPanicResult::eq_almost(&$lft, &$rgt) {
            panic!("{}", format!($($arg)+));
        }
    };
}

/// Asserts that two expressions are nearly equal (using [`eq_nearly`]).
///
/// [`eq_nearly`]: crate::TestPanicResult::eq_nearly
#[macro_export]
macro_rules! assert_eqn {
    ($lft:expr, $rgt:expr $(,)?) => {
        if !$crate::TestPanicResult::eq_nearly(&$lft, &$rgt) {
            panic!("{}", $crate::msg::eqn_failed(&$lft, &$rgt));
        }
    };

    ($lft:expr, $rgt:expr, $($arg:tt)+) => {
        if !$crate::TestPanicResult::eq_nearly(&$lft, &$rgt) {
            panic!("{}", format!($($arg)+));
        }
    };
}

pub use assert_eqa;
pub use assert_eqn;
