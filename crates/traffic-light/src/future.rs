//! Extension trait for [`Future`].
//!
//! # Examples
//!
//! ```
//! use std::{
//!     error,
//!     result
//! };
//!
//! use traffic_light::future::FutureExt as _;
//!
//! fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//!     async {
//!         // ...
//!         Ok::<(), Box<dyn std::error::Error>>(())
//!     }.block_on()?;
//!
//!     Ok(())
//! }
//! ```

use crate::executor::Executor;

/// An extension trait for [`Future`] that provides blocking.
pub trait FutureExt: Future {
    /// Blocks the current thread on this future.
    ///
    /// # Panics
    ///
    /// Panics if this future panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::{
    /// #     error,
    /// #     result
    /// # };
    /// #
    /// use traffic_light::future::FutureExt as _;
    ///
    /// # fn main() -> result::Result<(), Box<dyn error::Error>> {
    /// let x: result::Result<(), Box<dyn error::Error>> =
    ///     async {
    ///         // ...
    ///         Ok(())
    ///     }.block_on();
    ///
    /// assert!(x.is_ok());
    /// #
    /// #     Ok(())
    /// # }
    /// ```
    fn block_on(self) -> Self::Output;
}

impl<F: Future> FutureExt for F {
    fn block_on(self) -> Self::Output {
        Executor::block_on(self)
    }
}

#[cfg(test)]
mod tests {
    #[rustfmt::skip]
    use std::{
        error,
        result,
    };

    use super::*;

    #[rustfmt::skip]
    #[test]
    fn block_on_result_ok() {
        let x: result::Result<(), Box<dyn error::Error>> =
            async {
                Ok(())
            }.block_on();

        assert!(x.is_ok());
    }

    #[rustfmt::skip]
    #[test]
    fn block_on_result_err() {
        let x: result::Result<(), &str> =
            async {
                Err("")
            }.block_on();

        assert!(x.is_err());
    }
}
