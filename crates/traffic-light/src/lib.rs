//! Another single-threaded block asynchronous executor for Rust.
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
//! fn main() -> result::Result<(), Box<dyn error::Error>> {
//!     async {
//!         // ...
//!         Ok::<(), Box<dyn error::Error>>(())
//!     }.block_on()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ```
//! use std::{
//!     error,
//!     result
//! };
//!
//! use traffic_light::executor::Executor;
//!
//! fn main() -> result::Result<(), Box<dyn error::Error>> {
//!     Executor::block_on(async {
//!         // ...
//!         Ok::<(), Box<dyn error::Error>>(())
//!     })?;
//!
//!     Ok(())
//! }
//! ```

pub mod executor;
pub mod future;
pub mod prelude;
pub(crate) mod thread;
