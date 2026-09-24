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
use syn::{ItemFn, parse::Nothing, parse_macro_input, parse_quote};

/// Expands `async fn main() {}` into a call to [`Executor::block_on`].
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
/// ```ignore
/// fn main() -> result::Result<(), Box<dyn error::Error>> {
///     ::traffic_light::executor::Executor::block_on(async {
///         // ...
///         Ok(())
///     })
/// }
/// ```
///
/// [`Executor::block_on`]: ../traffic_light/executor/struct.Executor.html#method.block_on
#[proc_macro_attribute]
pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(attr as Nothing);
    let item = parse_macro_input!(item as ItemFn);

    if item.sig.ident != "main" {
        return syn::Error::new_spanned(
            &item.sig.ident,
            "the `#[traffic_light::main]` attribute may only be used on `main`",
        )
        .to_compile_error()
        .into();
    }

    expand_async_fn(item)
}

#[proc_macro_attribute]
pub fn test(attr: TokenStream, item: TokenStream) -> TokenStream {
    parse_macro_input!(attr as Nothing);
    let mut item = parse_macro_input!(item as ItemFn);

    item.attrs
        .push(parse_quote! { #[::std::prelude::v1::test] });

    expand_async_fn(item)
}

fn expand_async_fn(item: ItemFn) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = item;

    if sig.asyncness.take().is_none() {
        return syn::Error::new_spanned(sig.fn_token, "function must be `async`")
            .to_compile_error()
            .into();
    }

    let expanded = quote! {
        #(#attrs)*
        #vis #sig {
            ::traffic_light::executor::Executor::block_on(async #block)
        }
    };

    TokenStream::from(expanded)
}
