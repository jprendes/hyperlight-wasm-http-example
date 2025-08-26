use crate::{
    bindings::wasi,
    resource::{BlockOn as _, Resource},
};

use super::{WasiImpl, headers::Headers, http_incoming_body::IncomingBody};

pub struct IncomingRequest {
    pub method: wasi::http::types::Method,
    pub path_with_query: Option<alloc::string::String>,
    pub scheme: Option<wasi::http::types::Scheme>,
    pub authority: Option<alloc::string::String>,
    pub headers: Resource<Headers>,
    pub body: Resource<IncomingBody>,
    pub body_taken: bool,
}

impl wasi::http::types::IncomingRequest<Resource<Headers>, Resource<IncomingBody>> for WasiImpl {
    type T = Resource<IncomingRequest>;

    fn method(
        &mut self,
        self_: hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
    ) -> wasi::http::types::Method {
        self_.read().block_on().method.clone()
    }

    fn path_with_query(
        &mut self,
        self_: hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
    ) -> Option<alloc::string::String> {
        self_.read().block_on().path_with_query.clone()
    }

    fn scheme(
        &mut self,
        self_: hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
    ) -> Option<wasi::http::types::Scheme> {
        self_.read().block_on().scheme.clone()
    }

    fn authority(
        &mut self,
        self_: hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
    ) -> Option<alloc::string::String> {
        self_.read().block_on().authority.clone()
    }

    fn headers(
        &mut self,
        self_: hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
    ) -> wasi::http::types::Headers<Resource<Headers>> {
        self_.read().block_on().headers.clone()
    }

    fn consume(
        &mut self,
        self_: hyperlight_common::resource::BorrowedResourceGuard<Self::T>,
    ) -> Result<Resource<IncomingBody>, ()> {
        let mut self_ = self_.write().block_on();
        if self_.body_taken {
            return Err(());
        }
        self_.body_taken = true;
        Ok(self_.body.clone())
    }
}
