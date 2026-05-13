# traffic-light-rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/traffic-light.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/traffic-light)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-traffic--light-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/traffic-light)
[<img alt="github" src="https://img.shields.io/badge/github-red--taxicab/traffic--light--rs-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/red-taxicab/traffic-light-rs)

Another single-threaded blocking asynchronous executor for Rust.

## Examples

```rust
use std::{
    error,
    pin::Pin,
    result,
    task::{Context, Poll},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

enum State {
    Idle,
    Spawned(JoinHandle<()>),
    Completed,
}

pub struct Timer {
    state: State,
    deadline: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            state: State::Idle,
            deadline: (Instant::now() + duration),
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.state {
            State::Idle => {
                let deadline = self.deadline;

                let waker = cx.waker().clone();

                let join_handle = thread::spawn(move || {
                    thread::sleep(deadline.saturating_duration_since(Instant::now()));

                    waker.wake();
                });

                self.state = State::Spawned(join_handle);

                Poll::Pending
            },
            State::Spawned(join_handle) => {
                if join_handle.is_finished() {
                    self.state = State::Completed;

                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            },
            State::Completed => Poll::Ready(()),
        }
    }
}

#[traffic_light::main]
async fn main() -> result::Result<(), Box<dyn error::Error>> {
    Timer::new(Duration::from_secs(5)).await;

    println!("Hello, world!");

    Ok(())
}

```

## Features

- `macros` adds `[traffic_light::main]`

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
