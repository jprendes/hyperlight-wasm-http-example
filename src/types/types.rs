use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{bindings::wasi, resource::Resource};

use super::{WasiImpl, io_poll::AnyPollable, io_stream::Stream};

impl
    wasi::http::Types<u64, anyhow::Error, Resource<Stream>, Resource<Stream>, Resource<AnyPollable>>
    for WasiImpl
{
    fn http_error_code(
        &mut self,
        err: BorrowedResourceGuard<anyhow::Error>,
    ) -> Option<wasi::http::types::ErrorCode> {
        Some(wasi::http::types::ErrorCode::InternalError(Some(
            err.to_string(),
        )))
    }
}
