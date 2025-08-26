use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi,
    resource::{BlockOn, Resource},
};

use super::{WasiImpl, headers::Headers, http_outgoing_body::OutgoingBody};

pub struct OutgoingResponse {
    pub status_code: u16,
    pub headers: Resource<Headers>,
    pub body: Resource<OutgoingBody>,
    body_taken: bool,
}

impl wasi::http::types::OutgoingResponse<Resource<Headers>, Resource<OutgoingBody>> for WasiImpl {
    type T = Resource<OutgoingResponse>;

    fn new(&mut self, headers: Resource<Headers>) -> Self::T {
        Resource::new(OutgoingResponse {
            status_code: 200,
            headers: headers,
            body: Resource::default(),
            body_taken: false,
        })
    }

    fn status_code(&mut self, self_: BorrowedResourceGuard<Self::T>) -> u16 {
        self_.read().block_on().status_code
    }

    fn set_status_code(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        status_code: u16,
    ) -> Result<(), ()> {
        self_.write().block_on().status_code = status_code;
        Ok(())
    }

    fn headers(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<Headers> {
        self_.read().block_on().headers.clone()
    }

    fn body(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<Resource<OutgoingBody>, ()> {
        let mut self_ = self_.write().block_on();
        if self_.body_taken {
            return Err(());
        }
        self_.body_taken = true;
        Ok(self_.body.clone())
    }
}
