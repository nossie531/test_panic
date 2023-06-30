use std::panic;
use std::panic::panic_any;
use test_panic::{test_panic, TestPanicResult};

use crate::for_test::cast;

#[test]
fn test_panic_with_cool() {
    let result = test_panic(|| {});

    assert!(result.is_cool());
    assert!(!result.is_panic());
    dbg!("OK");
}

#[test]
fn test_panic_with_panic() {
    let result = test_panic(|| panic!());

    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert_eq!(cast::<&str>(result.payload()), &result.message().as_str());
}

#[test]
fn test_panic_with_panic_str() {
    let result = test_panic(|| panic!("panic"));

    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert_eq!(result.message(), "panic");
    assert_eq!(cast::<&str>(result.payload()).to_owned(), "panic");
}

#[test]
fn test_panic_with_panic_string() {
    let result = test_panic(|| panic!("{}", "panic"));

    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert_eq!(result.message(), "panic");
    assert_eq!(cast::<String>(result.payload()), "panic");
}

#[test]
fn test_panic_with_panic_any() {
    let result = test_panic(|| panic_any(false));

    assert!(!result.is_cool());
    assert!(result.is_panic());
    assert_eq!(*cast::<bool>(result.payload()), false);
}

#[test]
#[should_panic]
fn test_panic_result_payload_with_cool() {
    panic::set_hook(Box::new(|_| {}));
    let target = TestPanicResult::Cool;

    _ = target.payload();
}

#[test]
#[should_panic]
fn test_panic_result_message_with_cool() {
    panic::set_hook(Box::new(|_| {}));
    let target = TestPanicResult::Cool;

    _ = target.message();
}

#[test]
#[should_panic]
fn test_panic_result_message_with_none_str() {
    panic::set_hook(Box::new(|_| {}));
    let target = TestPanicResult::Panic(Box::new(false));

    _ = target.message();
}

mod for_test {
    use std::any::Any;

    pub fn cast<T: 'static>(val: &Box<dyn Any + Send>) -> &T {
        val.downcast_ref::<T>().unwrap()
    }
}
