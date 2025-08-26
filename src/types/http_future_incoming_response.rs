use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi,
    resource::{BlockOn as _, Resource},
};

use super::{
    WasiImpl, http_future::FutureHttp, http_incoming_response::IncomingResponse,
    io_poll::AnyPollable,
};

pub type FutureIncomingResponse =
    FutureHttp<Result<Resource<IncomingResponse>, wasi::http::types::ErrorCode>>;

impl wasi::http::types::FutureIncomingResponse<Resource<IncomingResponse>, Resource<AnyPollable>>
    for WasiImpl
{
    type T = Resource<FutureIncomingResponse>;

    fn subscribe(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<AnyPollable> {
        self_.poll(|r| r.is_ready())
    }

    fn get(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Option<Result<Result<Resource<IncomingResponse>, wasi::http::types::ErrorCode>, ()>> {
        self_.write().block_on().get()
    }
}
