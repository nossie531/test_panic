mod for_test;

use for_test::dyn_eq;
use for_test::suppress_stderr;
use test_panic::TestPanicResult;

#[test]
fn cool() {
    with_cool();
    with_panic();

    fn with_cool() {
        let target = TestPanicResult::Cool("ok");

        let result = target.cool();

        assert!(result.is_some_and(|x| x == "ok"));
    }

    fn with_panic() {
        let target = TestPanicResult::<()>::Panic(Box::new(""));

        let result = target.cool();

        assert!(result.is_none());
    }
}

#[test]
fn panic() {
    with_cool();
    with_panic();

    fn with_cool() {
        let target = TestPanicResult::Cool(());

        let result = target.panic();

        assert!(result.is_none());
    }

    fn with_panic() {
        let target = TestPanicResult::<()>::Panic(Box::new("ng"));

        let result = target.panic();

        assert!(result.is_some_and(|x| dyn_eq(&x, &"ng")));
    }
}

#[test]
#[should_panic]
fn payload_with_cool() {
    suppress_stderr();
    let target = TestPanicResult::Cool(());

    _ = target.payload();
}

#[test]
#[should_panic]
fn message_with_cool() {
    suppress_stderr();
    let target = TestPanicResult::Cool(());

    _ = target.message();
}

#[test]
#[should_panic]
fn message_with_none_str() {
    suppress_stderr();
    let target = TestPanicResult::<()>::Panic(Box::new(false));

    _ = target.message();
}
