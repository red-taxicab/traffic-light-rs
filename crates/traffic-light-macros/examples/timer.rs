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
            }
            State::Spawned(join_handle) => {
                if join_handle.is_finished() {
                    self.state = State::Completed;

                    Poll::Ready(())
                } else {
                    Poll::Pending
                }
            }
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
