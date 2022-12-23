use core::pin::Pin;
use core::{
    future::Future,
    task::{Context, Poll},
};

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub struct JoinedFuture<'a, T> {
    futures: Vec<(Option<T>, BoxFuture<'a, T>)>,
}

impl<'a, T> JoinedFuture<'a, T> {
    #[inline]
    fn new(futures: Vec<BoxFuture<'a, T>>) -> Self {
        Self {
            futures: futures.into_iter().map(|x| (None, x)).collect(),
        }
    }
}

impl<'a, T> Future for JoinedFuture<'a, T> {
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
            if let Poll::Ready(content) = future.1.as_mut().poll(cx) {
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
pub fn prep<'a, T>(future: impl Future<Output = T> + 'a) -> BoxFuture<'a, T> {
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

#[macro_export]
macro_rules! join_boxed {
    ($($a:expr),* $(,)?) => {
        join(vec![$(
            $a,
        )*])
    };
}
