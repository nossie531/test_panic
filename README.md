# test_panic

Utility for test cases with panic.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

Provides functions for test with panic. For the same purpose, the `shoud_panic`
attribute is provided in the Rust standard, but it is not so useful, hence we
created this crate.

## Examples

Example with this crate.

```rust
use test_panic::prelude::*;

#[test]
fn test() {
    let result = test_panic(|| panic!("message."));
    assert!(result.is_panic());
    assert!(result.message().contains("message."));
}
```

Example with `should_panic`.

```rust
#[test]
#[should_panic(expected = "message.")]
fn test() {
    // Suppresses standard error output.
    panic::set_hook(Box::new(|_| {}));
    panic!("message.");
}
```

## History

See [CHANGELOG](CHANGELOG.md).
