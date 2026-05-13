//! Macros for another single-threaded blocking asynchronous executor for Rust.
//!
//! # Examples
//!
//! ```
//! use std::{
//!     error,
//!     result,
//! };
//!
//! #[traffic_light::main]
//! async fn main() -> result::Result<(), Box<dyn error::Error>> {
//!     // ...
//!     Ok(())
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, parse_quote};

/// Expands `async fn main() {}` into a call to [`Executor::block_on`]
///
/// # Examples
///
/// ```
/// use std::{
///     error,
///     result,
/// };
///
/// #[traffic_light::main]
/// async fn main() -> result::Result<(), Box<dyn error::Error>> {
///     // ...
///     Ok(())
/// }
/// ```
///
/// # Expansion
///
/// ```
/// fn main() {
///     ::traffic_light::executor::Executor::block_on(async {
///         // ...
///     })
/// }
/// ```
///
/// [`Executor::block_on`]: ../traffic-light/executor/struct.Executor.html#method.block_on
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    expand_async_fn(item)
}

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ItemFn);

    item.attrs.push(parse_quote! { #[test] });

    expand_async_fn(item)
}

fn expand_async_fn(item: ItemFn) -> TokenStream {
    let ItemFn {
        attrs,
        mut sig,
        block,
        ..
    } = item;

    sig.asyncness = None;

    let expanded = quote! {
        #(#attrs)*
        #sig {
            ::traffic_light::executor::Executor::block_on(async #block)
        }
    };

    TokenStream::from(expanded)
}
