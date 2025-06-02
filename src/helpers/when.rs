/// Conditional value resolver. Evaluates one of two expressions based on the condition.
pub fn when<T, C>(
    condition: C,
    value_expr: impl IntoValueOrFn<T>,
    default_expr: impl IntoValueOrFn<T>,
) -> T
where
    C: IntoCondition,
{
    if condition.into_bool() {
        value(value_expr.into())
    } else {
        value(default_expr.into())
    }
}
