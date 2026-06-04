use proc_macro::TokenStream;

/// Derive macro that auto-implements the `Describe` trait.
///
/// Generates a `describe()` method that returns a string like:
/// `"StructName { field1: Type1, field2: Type2 }"`
///
/// # Example
///
/// ```ignore
/// use smart_pointer_workshop::Describe;
///
/// #[derive(Describe)]
/// struct Point {
///     x: f64,
///     y: f64,
/// }
///
/// let p = Point { x: 1.0, y: 2.0 };
/// assert_eq!(p.describe(), "Point { x: f64, y: f64 }");
/// ```
#[proc_macro_derive(Describe)]
pub fn derive_describe(_input: TokenStream) -> TokenStream {
    todo!("Students implement this derive macro")
}
