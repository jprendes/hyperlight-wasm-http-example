use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi,
    resource::{BlockOn, Resource},
};

use super::{WasiImpl, headers::Headers, http_incoming_body::IncomingBody};

pub struct IncomingResponse {
    pub status: wasi::http::types::StatusCode,
    pub headers: Resource<Headers>,
    pub body: Resource<IncomingBody>,
    pub body_taken: bool,
}

impl wasi::http::types::IncomingResponse<Resource<Headers>, Resource<IncomingBody>> for WasiImpl {
    type T = Resource<IncomingResponse>;

    fn status(&mut self, self_: BorrowedResourceGuard<Self::T>) -> wasi::http::types::StatusCode {
        self_.read().block_on().status
    }

    fn headers(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<Headers> {
        self_.read().block_on().headers.clone()
    }

    fn consume(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<Resource<IncomingBody>, ()> {
        let mut this = self_.write().block_on();
        if this.body_taken {
            return Err(());
        }
        this.body_taken = true;
        Ok(this.body.clone())
    }
}
