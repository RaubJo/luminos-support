#[macro_export]
macro_rules! fluent {
    ( $( $key:ident : $value:expr ),* $(,)? ) => {{
        let mut temp = $crate::Fluent::new();
        $(
            temp.set(stringify!($key), $value);
        )*
        temp
    }};
}
