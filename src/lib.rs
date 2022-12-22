use std::{sync::{Arc, mpsc::{SyncSender, sync_channel}, Mutex}, future::Future, task::Context, pin::Pin};

use futures_task::{ArcWake, waker_ref, Poll};

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

struct MicroTask<T> {
    future: Mutex<Option<BoxFuture<T>>>,
    queue: SyncSender<Arc<Self>>,
}

impl<T> ArcWake for MicroTask<T> {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // SAFETY: Can't fail because the runner only stops once the future is done.
        arc_self.queue.send(arc_self.clone()).unwrap(); 
    }
}

// SAFETY: These will never actually be used! This runner is single-threaded.
unsafe impl<T> Send for MicroTask<T> {}
unsafe impl<T> Sync for MicroTask<T> {}

pub fn sync<T>(future: impl Future<Output = T> + 'static) -> Option<T> {
    // Initialize things
    let (queue_sender, queue) = sync_channel(1); // Single-task runner, so this can be 1
    let future = Arc::new(MicroTask { future: Mutex::new(Some(Box::pin(future))), queue: queue_sender.clone() } );
    queue_sender.send(future).unwrap(); // SAFETY: Can't fail due to queue being in-scope

    // Now actually run the future.
    while let Ok(task) = queue.recv() {
        // SAFETY: This will never panic because this runner is single-threaded.
        if let Some(future) = task.future.lock().unwrap().as_mut() {
            let waker = waker_ref(&task);
            let context = &mut Context::from_waker(&waker);
            // If the future is done, stop and return.
            if let Poll::Ready(content) = future.as_mut().poll(context) {
                return Some(content);
            }
        }
    }

    None
}
