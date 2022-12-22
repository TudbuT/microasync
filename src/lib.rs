use std::{future::Future, task::Context, thread};

use futures_task::{Poll, noop_waker_ref};

pub fn sync<T>(future: impl Future<Output = T> + 'static) -> T {
    // Initialize things
    let mut future = Box::pin(future);

    // Now actually run the future.
    loop {
        // Usage of thread::yield_now combined with this being a single-task runtime makes the
        // waker redundant.
        let context = &mut Context::from_waker(noop_waker_ref());
        // If the future is done, stop and return.
        if let Poll::Ready(content) = future.as_mut().poll(context) {
            return content;
        }
        thread::yield_now();
    }
}
