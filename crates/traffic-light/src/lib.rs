//! Another single-threaded block asynchronous executor for Rust.
//!
//! # Examples
//!
//! ```
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
//!
//! ```
//! use traffic_light::executor::Executor;
//!
//! fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//!     Executor::block_on(async {
//!         // ...
//!         Ok::<(), Box<dyn std::error::Error>>(())
//!     })?;
//!
//!     Ok(())
//! }
//! ```

pub mod executor;
pub mod future;
pub mod prelude;
pub(crate) mod thread;
