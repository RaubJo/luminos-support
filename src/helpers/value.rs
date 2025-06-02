use std::collections::HashMap;

/// Represents either a value or a closure producing that value.
pub enum ValueOrFn<T> {
    Value(T),
    Closure(Box<dyn FnOnce() -> T>),
}

/// Resolves a `ValueOrFn<T>` by either returning the value or calling the closure.
pub fn value<T>(input: ValueOrFn<T>) -> T {
    match input {
        ValueOrFn::Value(val) => val,
        ValueOrFn::Closure(f) => f(),
    }
}

/// Trait to convert values or closures into `ValueOrFn<T>` safely.
pub trait IntoValueOrFn<T> {
    fn into(self) -> ValueOrFn<T>;
}

impl<T> IntoValueOrFn<T> for T {
    fn into(self) -> ValueOrFn<T> {
        ValueOrFn::Value(self)
    }
}

impl<T, F> IntoValueOrFn<T> for F
where
    F: FnOnce() -> T + 'static,
{
    fn into(self) -> ValueOrFn<T> {
        ValueOrFn::Closure(Box::new(self))
    }
}

/// Trait to allow `bool` or closures that return bool as conditions.
pub trait IntoCondition {
    fn into_bool(self) -> bool;
}

impl IntoCondition for bool {
    fn into_bool(self) -> bool {
        self
    }
}

impl<F> IntoCondition for F
where
    F: FnOnce() -> bool,
{
    fn into_bool(self) -> bool {
        self()
    }
}
