use crate::{
    bindings::wasi,
    resource::{BlockOn, Resource},
};

use super::{WasiImpl, http_future_headers::FutureHeaders, io_stream::Stream};

#[derive(Default)]
pub struct IncomingBody {
    pub stream: Resource<Stream>,
    pub trailers: Resource<FutureHeaders>,
    pub stream_taken: bool,
}

impl wasi::http::types::IncomingBody<Resource<FutureHeaders>, Resource<Stream>> for WasiImpl {
    type T = Resource<IncomingBody>;

    fn stream(
        &mut self,
        self_: hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
    ) -> Result<Resource<Stream>, ()> {
        let mut this = self_.write().block_on();
        if this.stream_taken {
            return Err(());
        }
        this.stream_taken = true;
        Ok(this.stream.clone())
    }

    fn finish(&mut self, this: Self::T) -> Resource<FutureHeaders> {
        this.read().block_on().trailers.clone()
    }
}
