#[macro_export]
macro_rules! value {
    // Handles value!(move || ...) or value!(|| ...)
    (move || $($body:tt)*) => {
        $crate::value($crate::helpers::value::ValueOrFn::Closure(Box::new(move || $($body)*)))
    };
    (|| $($body:tt)*) => {
        $crate::value($crate::helpers::value::ValueOrFn::Closure(Box::new(|| $($body)*)))
    };

    // Handles value!(function, args...) for callables
    ($func:expr, $($arg:expr),+ $(,)?) => {
        $crate::value($crate::helpers::value::ValueOrFn::Closure(Box::new(move || $func($($arg),+))))
    };

    // Handles value!(some_plain_value)
    ($val:expr) => {
        $crate::value($crate::helpers::value::ValueOrFn::Value($val))
    };
}
