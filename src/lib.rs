use std::{future::Future, task::{Context, Waker, RawWaker, RawWakerVTable, Poll}, thread, ptr::null};
use core::pin::Pin;

fn empty(_: *const ()) {}
fn empty_clone(_: *const ()) -> RawWaker {
    empty_raw_waker()
}

fn empty_raw_waker() -> RawWaker {
    RawWaker::new(null(), &RawWakerVTable::new(empty_clone, empty, empty, empty))
}

fn empty_waker() -> Waker {
    // SAFETY: Can't fail because no dynamic data is used and this function was tested.
    unsafe { Waker::from_raw(empty_raw_waker()) }
}

pub fn sync<T>(mut future: impl Future<Output = T> + 'static) -> T {
    // Initialize things
    // SAFETY: Safe because of use-case
    let mut future = unsafe {
        Pin::new_unchecked(&mut future)
    };

    // Now actually run the future.
    loop {
        // Usage of thread::yield_now combined with this being a single-task runtime makes the
        // waker redundant.
        let waker = empty_waker();
        let context = &mut Context::from_waker(&waker);
        // If the future is done, stop and return.
        if let Poll::Ready(content) = future.as_mut().poll(context) {
            return content;
        }
        thread::yield_now();
    }
}
