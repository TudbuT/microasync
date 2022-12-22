use core::pin::Pin;
use core::{
    future::Future,
    task::{Context, Poll},
};

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

pub struct JoinedFuture<T> {
    futures: Vec<(Option<T>, BoxFuture<T>)>,
}

impl<T> JoinedFuture<T> {
    #[inline]
    fn new(futures: Vec<BoxFuture<T>>) -> Self {
        Self {
            futures: futures.into_iter().map(|x| (None, x)).collect(),
        }
    }
}

impl<T> Future for JoinedFuture<T> {
    type Output = Vec<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut done = true;
        let me = unsafe {
            // SAFETY: This is the only way to access the futures array, and is safe because the
            // pin necessarily lives as long as poll() and is owned, so won't be modified
            self.get_unchecked_mut()
        };
        for future in me.futures.iter_mut() {
            if future.0.is_some() {
                continue;
            }
            done = false;
            if let Poll::Ready(content) = future.1.as_mut().as_mut().poll(cx) {
                future.0 = Some(content);
            }
        }
        if done {
            Poll::Ready(me.futures.iter_mut().map(|x| x.0.take().unwrap()).collect())
        } else {
            Poll::Pending
        }
    }
}

#[inline]
pub fn prep<T>(future: impl Future<Output = T> + 'static) -> BoxFuture<T> {
    Box::pin(future)
}

#[inline]
pub fn join<T>(futures: Vec<BoxFuture<T>>) -> JoinedFuture<T> {
    JoinedFuture::new(futures)
}

#[macro_export]
macro_rules! join {
    ($($a:expr),* $(,)?) => {
        join(vec![$(
            prep($a),
        )*])
    };
}
