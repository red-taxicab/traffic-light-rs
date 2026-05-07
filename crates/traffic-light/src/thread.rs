use std::{sync::Arc, task::Wake};

pub(crate) struct Thread(std::thread::Thread);

impl Default for Thread {
    fn default() -> Self {
        Self(std::thread::current())
    }
}

impl Wake for Thread {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}
