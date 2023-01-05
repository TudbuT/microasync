#![no_std]

#[cfg(not(feature = "no_joiner"))]
mod joiner;
#[cfg(not(feature = "no_joiner"))]
pub use joiner::{join, prep, BoxFuture};

use core::pin::Pin;
use core::{
    future::Future,
    ptr::null,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

fn empty(_: *const ()) {}
fn empty_clone(_: *const ()) -> RawWaker {
    empty_raw_waker()
}

#[inline]
fn empty_raw_waker() -> RawWaker {
    RawWaker::new(
        null(),
        &RawWakerVTable::new(empty_clone, empty, empty, empty),
    )
}

#[inline]
fn empty_waker() -> Waker {
    // SAFETY: Can't fail because no dynamic data is used and this function was tested.
    unsafe { Waker::from_raw(empty_raw_waker()) }
}

pub fn sync_with<T>(mut future: impl Future<Output = T>, poll_delay: u64) -> T {
    // Initialize things
    // SAFETY: Safe because this is single threaded and `future` won't be dropped.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

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
        #[cfg(not(feature = "no_std"))]
        {
            // this won't be added to the binary if the no_std feature is enabled.
            extern crate std;
            // thread::yield_now is bug-ridden, this'll have to do.
            std::thread::sleep(std::time::Duration::from_millis(poll_delay));
        }
    }
}

pub fn sync<T>(future: impl Future<Output = T>) -> T {
    sync_with(future, 1)
}
