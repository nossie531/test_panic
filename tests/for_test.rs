#![allow(unused)]

use std::any::Any;
use std::panic;

pub fn cast<T: Any>(val: &Box<dyn Any + Send>) -> &T {
    val.downcast_ref::<T>().unwrap()
}

pub fn dyn_eq<T: Any + Eq>(x: &Box<dyn Any + Send>, y: &T) -> bool {
    cast::<T>(x) == y
}

pub fn suppress_stderr() {
    panic::set_hook(Box::new(|_| {}));
}
