test_panic
===

Utility for test cases with panic.

*The author of this crate is not good at English.*  
*Forgive me if the document is hard to read.*

## What is this?

Provides functions for test with panic. For the same purpose, the `shoud_panic`
attribute is provided in the Rust standard, but it is not so useful, hence we
created this crate.

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
    // Suppresses standard error output.
    panic::set_hook(Box::new(|_| {}));

    panic!("message.");
}
```

# What's New

At Version 0.2.0.

* Some document is polished.
* `TestPanicResult` holds value on cases where no panic occurred.
