use std::{cell::UnsafeCell, future::Future, pin::Pin, sync::Arc, task::Context, thread};

use futures_task::{waker_ref, ArcWake, Poll};

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

struct MicroTask<T> {
    future: UnsafeCell<BoxFuture<T>>,
    sync_thread: thread::Thread,
}

impl<T> ArcWake for MicroTask<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // SAFETY: Can't fail because the runner only stops once the future is done.
        arc_self.sync_thread.unpark();
    }
}

// SAFETY: These will never actually be used! This runner is single-threaded.
unsafe impl<T> Send for MicroTask<T> {}
unsafe impl<T> Sync for MicroTask<T> {}

pub fn sync<T>(future: impl Future<Output = T> + 'static) -> T {
    // Initialize things
    let task = Arc::new(MicroTask {
        future: UnsafeCell::new(Box::pin(future)),
        sync_thread: thread::current(),
    });

    // Now actually run the future.
    loop {
        // SAFETY: Access to the future field is single-threaded (unlike the sync_thread field),
        // therefore we can access it mutably.
        let future = unsafe { task.future.get().as_mut().unwrap() };
        let waker = waker_ref(&task);
        let context = &mut Context::from_waker(&waker);
        // If the future is done, stop and return.
        if let Poll::Ready(content) = future.as_mut().poll(context) {
            return content;
        }
        thread::park();
    }
}
