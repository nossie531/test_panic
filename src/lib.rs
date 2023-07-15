/*! Utility for test cases with panic.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

For the same purpose, the `shoud_panic` attribute is provided in the Rust
standard, but it is not so useful, hence we created this crate.

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

mod test_panic;
mod test_panic_result;

pub use crate::test_panic::test_panic;
pub use test_panic_result::TestPanicResult;
