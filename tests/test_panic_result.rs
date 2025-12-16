mod for_test;

use for_test::*;
use test_panic::*;

#[test]
fn is_cool() {
    with_false();
    with_true();

    fn with_false() {
        let target = TestPanicResult::<()>::Panic(Box::new(()));
        let result = target.is_cool();
        assert!(!result);
    }

    fn with_true() {
        let target = TestPanicResult::Cool(());
        let result = target.is_cool();
        assert!(result);
    }
}

#[test]
fn is_panic() {
    with_false();
    with_true();

    fn with_false() {
        let target = TestPanicResult::Cool(());
        let result = target.is_panic();
        assert!(!result);
    }

    fn with_true() {
        let target = TestPanicResult::<()>::Panic(Box::new(()));
        let result = target.is_panic();
        assert!(result);
    }
}

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
fn value() {
    let target = TestPanicResult::Cool(42);
    let result = target.value();
    assert_eq!(result, &42);
}

#[test]
fn payload() {
    let target = TestPanicResult::<()>::Panic(Box::new(42));
    let result = target.payload();
    assert_eq!(result.downcast_ref::<i32>().unwrap(), &42);
}

#[test]
fn message() {
    let target = TestPanicResult::<()>::Panic(Box::new("error"));
    let result = target.message();
    assert_eq!(result, "error");
}

#[test]
fn get_message() {
    with_str_payload();
    with_none_str_payload();

    fn with_str_payload() {
        let target = TestPanicResult::<()>::Panic(Box::new("error"));
        let result = target.get_message();
        assert_eq!(result, Some("error".to_string()));
    }

    fn with_none_str_payload() {
        let target = TestPanicResult::<()>::Panic(Box::new(42));
        let result = target.get_message();
        assert_eq!(result, None);
    }
}

#[test]
fn nearly_eq() {
    with_same_cool_vs_cool();
    with_diff_cool_vs_cool();
    with_same_panic_vs_panic();
    with_diff_panic_vs_panic();

    fn with_same_cool_vs_cool() {
        let target = TestPanicResult::Cool(42);
        let other = TestPanicResult::Cool(42);
        let result = target.nearly_eq(&other);
        assert!(result);
    }

    fn with_diff_cool_vs_cool() {
        let target = TestPanicResult::Cool(42);
        let other = TestPanicResult::Cool(43);
        let result = target.nearly_eq(&other);
        assert!(!result);
    }

    fn with_same_panic_vs_panic() {
        let target = TestPanicResult::<()>::Panic(Box::new("foo"));
        let other = TestPanicResult::<()>::Panic(Box::new("foo"));
        let result = target.nearly_eq(&other);
        assert!(result);
    }

    fn with_diff_panic_vs_panic() {
        let target = TestPanicResult::<()>::Panic(Box::new("foo"));
        let other = TestPanicResult::<()>::Panic(Box::new(false));
        let result = target.nearly_eq(&other);
        assert!(result);
    }
}

#[test]
fn eq() {
    with_same_cool_vs_cool();
    with_diff_cool_vs_cool();
    with_same_panic_vs_panic();
    with_diff_panic_vs_panic();
    with_none_str_panic_payload();

    fn with_same_cool_vs_cool() {
        let target = TestPanicResult::Cool(42);
        let other = TestPanicResult::Cool(42);
        let result = target.eq(&other);
        assert!(result);
    }

    fn with_diff_cool_vs_cool() {
        let target = TestPanicResult::Cool(42);
        let other = TestPanicResult::Cool(43);
        let result = target.eq(&other);
        assert!(!result);
    }

    fn with_same_panic_vs_panic() {
        let target = TestPanicResult::<()>::Panic(Box::new("foo"));
        let other = TestPanicResult::<()>::Panic(Box::new("foo"));
        let result = target.eq(&other);
        assert!(result);
    }

    fn with_diff_panic_vs_panic() {
        let target = TestPanicResult::<()>::Panic(Box::new("foo"));
        let other = TestPanicResult::<()>::Panic(Box::new("bar"));
        let result = target.eq(&other);
        assert!(!result);
    }

    fn with_none_str_panic_payload() {
        let target = TestPanicResult::<()>::Panic(Box::new(42));
        let other = TestPanicResult::<()>::Panic(Box::new(43));
        let result = target.eq(&other);
        assert!(result);
    }
}

#[test]
#[should_panic]
fn value_with_cool() {
    suppress_stderr();
    let target = TestPanicResult::<()>::Panic(Box::new(()));
    _ = target.value();
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
