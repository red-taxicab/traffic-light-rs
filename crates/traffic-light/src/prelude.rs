//! Re-exports.
//!
//! # Examples
//!
//! ```
//! use traffic_light::prelude::*;
//!
//! fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//!     Executor::block_on(async {
//!         async {
//!             // ...
//!             Ok::<(), Box<dyn std::error::Error>>(())
//!         }.block_on()?;
//!
//!         Ok::<(), Box<dyn std::error::Error>>(())
//!     })?;
//!
//!     Ok(())
//! }
//! ```

pub use crate::{
    executor::{self, Executor},
    future::{self, FutureExt},
};
