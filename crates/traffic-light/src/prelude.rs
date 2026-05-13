//! Re-exports.
//!
//! # Examples
//!
//! ```
//! use std::{
//!     error,
//!     result
//! };
//!
//! use traffic_light::prelude::*;
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
//! use traffic_light::prelude::*;
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

pub use crate::{
    executor::{self, Executor},
    future::{self, FutureExt},
};
