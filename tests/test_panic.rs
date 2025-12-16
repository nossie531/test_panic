mod for_test;

use for_test::*;
use std::panic;
use test_panic::*;

#[test]
fn with_cool_void() {
    let result = test_panic(|| {});

    assert!(result.is_cool());
    assert!(!result.is_panic());
    assert!(result.cool().is_some_and(|_: ()| true));
}

#[test]
fn with_cool_return() {
    let result = test_panic(|| "ok");

    assert!(result.is_cool());
    assert!(!result.is_panic());
    assert!(*result.value() == "ok");
    assert!(result.cool().is_some_and(|x| x == "ok"));
}

#[test]
fn with_panic_empty() {
    let result = test_panic(|| panic!());

    let def_msg = result.message().clone();
    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert!(cast::<String>(result.payload()) == &def_msg);
    assert!(
        result
            .panic()
            .is_some_and(|x| cast::<String>(&x) == &def_msg)
    );
}

#[test]
fn with_panic_str() {
    let result = test_panic(|| panic!("ng"));

    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert!(result.message() == "ng");
    assert!(dyn_eq(result.payload(), &"ng"));
    assert!(result.panic().is_some_and(|x| dyn_eq(&x, &"ng")));
}

#[test]
fn with_panic_string() {
    let result = test_panic(|| panic!("{}", "ng"));

    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert!(result.message() == "ng");
    assert!(dyn_eq::<String>(result.payload(), &"ng".to_owned()));
    assert!(result.panic().is_some_and(|x| dyn_eq(&x, &"ng".to_owned())));
}

#[test]
fn with_panic_any() {
    let result = test_panic(|| panic::panic_any(false));

    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert!(dyn_eq(result.payload(), &false));
    assert!(result.panic().is_some_and(|x| dyn_eq(&x, &false)));
}
