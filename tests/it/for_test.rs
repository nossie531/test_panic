use std::any::Any;
use std::panic;

/* # Vs generics monomorphization and coverage.
--
When you increase the number of type parameter variations, it tends
to lead to lower coverage. Therefore, base type is required.
-- */
pub type Ty = i32;

pub fn cast<T: Any>(val: &Box<dyn Any + Send>) -> &T {
    val.downcast_ref::<T>().unwrap()
}

pub fn dyn_eq<T: Any + Eq>(x: &Box<dyn Any + Send>, y: &T) -> bool {
    cast::<T>(x) == y
}

pub fn suppress_stderr() {
    panic::set_hook(Box::new(|_| {}));
}
