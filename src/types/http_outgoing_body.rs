use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi,
    resource::{BlockOn as _, Resource},
};

use wasi::http::types::ErrorCode;

use super::{WasiImpl, headers::Headers, io_stream::Stream};

#[derive(Default)]
pub struct OutgoingBody {
    content_length: Option<usize>,
    pub body: Resource<Stream>,
    pub trailers: Resource<Headers>,
    body_taken: bool,
    finished: bool,
}

impl OutgoingBody {
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn read_all(&mut self) -> Vec<u8> {
        self.body.write().block_on().read_all().unwrap_or_default()
    }
}

impl wasi::http::types::OutgoingBody<Resource<Headers>, Resource<Stream>> for WasiImpl {
    type T = Resource<OutgoingBody>;

    fn write(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Result<Resource<Stream>, ()> {
        let mut this = self_.write().block_on();
        if this.body_taken {
            return Err(());
        }
        this.body_taken = true;
        Ok(this.body.clone())
    }

    fn finish(
        &mut self,
        self_: Self::T,
        trailers: Option<Resource<Headers>>,
    ) -> Result<(), ErrorCode> {
        let mut this = self_.write().block_on();
        let (_, written) = this.body.write().block_on().close();
        this.finished = true;
        this.trailers = trailers.unwrap_or_default();
        if let Some(length) = this.content_length {
            if written != length {
                return Err(ErrorCode::HTTPRequestBodySize(Some(length as u64)));
            }
        }
        Ok(())
    }
}
