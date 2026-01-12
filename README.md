# test_panic

Utility for test cases with panic.

_The author of this crate is not good at English._  
_Forgive me if the document is hard to read._

## What is this?

Provides functions for test with panic. For the same purpose, the `shoud_panic`
attribute is provided in the Rust standard, but it is not so useful, hence we
created this crate.

## Example 1

Example with always panic.

```rust
use test_panic::prelude::*;

#[test]
fn test() {
    let result = test_panic(|| panic!("message."));
    assert!(result.is_panic());
    assert!(result.message().contains("message."));
}
```

Following is equivalent code with `should_panic`.

```rust
#[test]
#[should_panic(expected = "message.")]
fn test() {
    // Suppresses standard error output.
    panic::set_hook(Box::new(|_| {}));
    panic!("message.");
}
```

## Example 2

Example with multiple tests (This cannot be done with `should_panic`).

```rust
use test_panic::prelude::*;

#[test]
fn with_multi_tests() {
    let datas = [
        ((10, 3), ok(3)),
        ((10, 0), ng()),
        ((10, 15), msg("Result is too small")),
    ];

    for ((x, y), tobe) in datas {
        let asis = test_panic(|| divide(x, y));
        assert!(asis.almost_eq(&tobe));
    }
}

fn divide(x: i32, y: i32) -> i32 {
    assert!(y > 0);
    assert!(x / y >= 1, "Result is too small");
    x / y
}
```

## History

See [CHANGELOG](CHANGELOG.md).
