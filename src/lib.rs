/*! Utility for test cases with panic.

*The author of this crate is not good at English.*
*Forgive me if the document is hard to read.*

For the same purpose, the `shoud_panic` attribute is provided in the Rust
standard, but it is not so useful, hence we created this crate.

# Examples

```no_run
# // `no_run` is answer for `test_attr_in_doctest` of Clippy.
#[test]
fn test() {
    let result = test_panic(|| panic!("message."));
    assert!(result.is_panic());
    assert!(result.message().contains("message"));
}
```
*/

pub mod prelude;

mod functions;
mod test_panic_result;

pub use functions::*;
pub use test_panic_result::*;
