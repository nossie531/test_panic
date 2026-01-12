use crate::for_test::*;
use test_panic::prelude::*;

#[test]
fn assert_eqa_true() {
    let lft = ok(1);
    let rgt = ok(1);
    assert_eqa!(lft, rgt);
}

#[test]
#[should_panic(expected = "Assertion left and right almost equal failed.")]
fn assert_eqa_false() {
    suppress_stderr();
    let lft = ok(1);
    let rgt = ok(2);
    assert_eqa!(lft, rgt);
}

#[test]
#[should_panic(expected = "The custom message.")]
fn assert_eqa_false_with_custom_msg() {
    suppress_stderr();
    let lft = ok(1);
    let rgt = ok(2);
    assert_eqa!(lft, rgt, "The custom {}.", "message");
}

#[test]
fn assert_eqn_true() {
    let lft = ok(1);
    let rgt = ok(1);
    assert_eqn!(lft, rgt);
}

#[test]
#[should_panic(expected = "Assertion left and right nearly equal failed.")]
fn assert_eqn_false() {
    suppress_stderr();
    let lft = ok(1);
    let rgt = ok(2);
    assert_eqn!(lft, rgt);
}

#[test]
#[should_panic(expected = "The custom message.")]
fn assert_eqn_false_with_custom_msg() {
    suppress_stderr();
    let lft = ok(1);
    let rgt = ok(2);
    assert_eqa!(lft, rgt, "The custom {}.", "message");
}
