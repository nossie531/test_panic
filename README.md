test_panic
===

Utility for test cases with panic.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

Provides functions for test with panic. It allows for somewhat more
flexible testing than using the Rust standard `shoud_panic`.

## Examples

Example with this crate.

```rust
#[test]
fn test() {
    let result = test_panic(|| panic!("message."));

    assert!(result.is_panic());
    assert_eq!(result.message(), "message.");
}
```

Example with `should_panic`.

```rust
#[test]
#[should_panic(expected = "message.")]
fn test() {
    panic!("message.");
}
```