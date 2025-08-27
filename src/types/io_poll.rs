use std::{
    future::poll_fn,
    pin::Pin,
    task::{Context, Poll},
};

use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi::{self},
    resource::{BlockOn, Resource},
};

use super::WasiImpl;

struct PollableFuture<F: Future> {
    fut: Option<F>,
}

impl<F: Future> Future for PollableFuture<F> {
    type Output = bool;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<bool> {
        let this = unsafe { self.get_unchecked_mut() };
        let Some(fut) = this.fut.as_mut() else {
            return Poll::Ready(true);
        };
        let fut = unsafe { Pin::new_unchecked(fut) };
        match fut.poll(cx) {
            Poll::Pending => Poll::Ready(false),
            Poll::Ready(_) => {
                this.fut = None;
                Poll::Ready(true)
            }
        }
    }
}

pub struct AnyPollable {
    fut: PollableFuture<Pin<Box<dyn Future<Output = ()> + Send + Sync>>>,
}

impl AnyPollable {
    pub fn future(f: impl Future + Send + Sync + 'static) -> Self {
        let fut = async move {
            f.await;
        };
        let fut = PollableFuture {
            fut: Some(Box::pin(fut) as _),
        };
        Self { fut }
    }

    pub fn resource<T: Send + Sync + 'static>(
        res: Resource<T>,
        cond: impl Fn(&T) -> bool + Send + Sync + 'static,
    ) -> Self {
        Self::future(async move {
            res.read_wait_until(cond).await;
        })
    }

    pub async fn ready(&mut self) -> bool {
        let fut = &mut self.fut;
        let fut = Pin::new(fut);
        fut.await
    }

    pub async fn block(&mut self) {
        while !self.ready().await {}
    }

    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<bool> {
        let fut = &mut self.fut;
        let fut = Pin::new(fut);
        fut.poll(cx)
    }
}

impl wasi::io::poll::Pollable for WasiImpl {
    type T = Resource<AnyPollable>;

    fn ready(&mut self, self_: BorrowedResourceGuard<Self::T>) -> bool {
        self_.write().block_on().ready().block_on()
    }

    fn block(&mut self, self_: BorrowedResourceGuard<Self::T>) -> () {
        self_.write().block_on().block().block_on()
    }
}

impl wasi::io::Poll for WasiImpl {
    fn poll(&mut self, pollables: Vec<BorrowedResourceGuard<Resource<AnyPollable>>>) -> Vec<u32> {
        let mut pollables = pollables
            .into_iter()
            .map(|p| p.write().block_on())
            .collect::<Vec<_>>();

        poll_fn(move |cx| {
            for (i, pollable) in pollables.iter_mut().enumerate() {
                if let Poll::Ready(true) = pollable.poll(cx) {
                    return Poll::Ready(vec![i as u32]);
                }
            }
            Poll::Pending
        })
        .block_on()
    }
}
