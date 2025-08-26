use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi,
    resource::{BlockOn, Resource},
};

use super::{WasiImpl, headers::Headers, http_future::FutureHttp, io_poll::AnyPollable};

pub type FutureHeaders = FutureHttp<Result<Resource<Headers>, wasi::http::types::ErrorCode>>;

impl wasi::http::types::FutureTrailers<Resource<Headers>, Resource<AnyPollable>> for WasiImpl {
    type T = Resource<FutureHeaders>;

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        self_.poll(|r| r.is_ready())
    }

    fn get(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Option<Result<Result<Option<Resource<Headers>>, wasi::http::types::ErrorCode>, ()>> {
        match self_.write().block_on().get() {
            Some(Ok(Ok(headers))) if headers.read().block_on().is_empty() => Some(Ok(Ok(None))),
            Some(Ok(Ok(headers))) => Some(Ok(Ok(Some(headers)))),
            Some(Ok(Err(e))) => Some(Ok(Err(e))),
            Some(Err(())) => Some(Err(())),
            None => None,
        }
    }
}
