mod for_test;
mod funcs;
mod macros;
mod test_panic_result;

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
        assert_eqa!(asis, tobe);
    }
}

fn divide(x: i32, y: i32) -> i32 {
    assert!(y > 0);
    assert!(x / y >= 1, "Result is too small");
    x / y
}
