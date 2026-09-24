use std::{sync::Arc, task::Wake};

pub struct Thread(std::thread::Thread);

impl Thread {
    pub(crate) fn current() -> Self {
        Self(std::thread::current())
    }
}

impl Wake for Thread {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}
