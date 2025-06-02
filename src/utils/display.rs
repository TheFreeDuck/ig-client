#[macro_export]
/// Implements the Display trait for a type by serializing it to JSON
///
/// This macro automatically implements the Display trait for one or more types
/// by serializing them to JSON using serde_json. This is useful for debugging
/// and logging purposes.
///
/// # Examples
///
/// ```
/// use tracing::info;
/// use ig_client::impl_json_display;
///
/// #[derive(serde::Serialize)]
/// struct MyStruct {
///     field1: String,
///     field2: i32,
/// }
///
/// impl_json_display!(MyStruct);
///
/// let my_struct = MyStruct {
///     field1: "value".to_string(),
///     field2: 42,
/// };
///
/// info!("{}", my_struct); // Outputs JSON representation
/// ```
macro_rules! impl_json_display {
    ($($t:ty),+) => {
        $(
            impl std::fmt::Display for $t {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match serde_json::to_string(self) {
                        Ok(pretty_json) => write!(f, "{}", pretty_json),
                        Err(e) => write!(f, "Error serializing to JSON: {}", e),
                    }
                }
            }
        )+
    }
}
