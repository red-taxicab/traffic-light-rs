//! The traffic-light Executor.
//!
//! # Examples
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

use std::{
    pin::pin,
    sync::Arc,
    task::{Context, Poll, Waker},
    thread,
};

use crate::thread::Thread;

thread_local! {
    static WAKER: Waker = Waker::from(Arc::new(Thread::default()));
}

/// The traffic-light Executor.
pub struct Executor;

impl Executor {
    /// Blocks the current thread until the future is ready.
    ///
    /// # Panics
    ///
    /// Panics if the provided future panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use traffic_light::executor::Executor;
    ///
    /// let x: std::result::Result<(), Box<dyn std::error::Error>> =
    ///     Executor::block_on(async {
    ///         // ...
    ///         Ok(())
    ///     });
    ///
    /// assert!(x.is_ok());
    /// ```
    pub fn block_on<F: IntoFuture>(f: F) -> F::Output {
        let mut f = pin!(f.into_future());

        WAKER.with(|waker| {
            let mut cx = Context::from_waker(&waker);

            loop {
                match f.as_mut().poll(&mut cx) {
                    Poll::Ready(x) => return x,
                    Poll::Pending => thread::park(),
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_on_result_ok() {
        let x: std::result::Result<(), Box<dyn std::error::Error>> =
            Executor::block_on(async {
                Ok(())
            });

        assert!(x.is_ok());
    }

    #[test]
    fn block_on_result_err() {
        let x: std::result::Result<(), &str> =
            Executor::block_on(async {
                Err("")
            });

        assert!(x.is_err());
    }
}
