use hyperlight_common::resource::BorrowedResourceGuard;

use crate::{
    bindings::wasi,
    resource::{BlockOn as _, Resource},
};

use super::{WasiImpl, headers::Headers, http_outgoing_body::OutgoingBody};

pub struct OutgoingRequest {
    pub method: wasi::http::types::Method,
    pub path_with_query: Option<String>,
    pub scheme: Option<wasi::http::types::Scheme>,
    pub authority: Option<String>,
    pub headers: Resource<Headers>,
    pub body: Resource<OutgoingBody>,
    body_taken: bool,
}

impl wasi::http::types::OutgoingRequest<Resource<Headers>, Resource<OutgoingBody>> for WasiImpl {
    type T = Resource<OutgoingRequest>;

    fn new(&mut self, headers: Resource<Headers>) -> Self::T {
        Resource::new(OutgoingRequest {
            method: wasi::http::types::Method::Get,
            path_with_query: None,
            scheme: None,
            authority: None,
            headers,
            body: Resource::default(),
            body_taken: false,
        })
    }

    fn body(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Result<Resource<OutgoingBody>, ()> {
        let mut this = self_.write().block_on();
        if this.body_taken {
            return Err(());
        }
        this.body_taken = true;
        Ok(this.body.clone())
    }

    fn method(&mut self, self_: BorrowedResourceGuard<Self::T>) -> wasi::http::types::Method {
        self_.read().block_on().method.clone()
    }

    fn set_method(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        method: wasi::http::types::Method,
    ) -> Result<(), ()> {
        self_.write().block_on().method = method;
        Ok(())
    }

    fn path_with_query(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Option<String> {
        self_.read().block_on().path_with_query.clone()
    }

    fn set_path_with_query(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        path_with_query: Option<String>,
    ) -> Result<(), ()> {
        // TODO: validate the path_with_query
        self_.write().block_on().path_with_query = path_with_query;
        Ok(())
    }

    fn scheme(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
    ) -> Option<wasi::http::types::Scheme> {
        self_.read().block_on().scheme.clone()
    }

    fn set_scheme(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        scheme: Option<wasi::http::types::Scheme>,
    ) -> Result<(), ()> {
        // TODO: valudate the `Other` scheme
        self_.write().block_on().scheme = scheme;
        Ok(())
    }

    fn authority(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Option<String> {
        self_.read().block_on().authority.clone()
    }

    fn set_authority(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        authority: Option<String>,
    ) -> Result<(), ()> {
        // TODO: validate the authority
        self_.write().block_on().authority = authority;
        Ok(())
    }

    fn headers(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Resource<Headers> {
        self_.read().block_on().headers.clone()
    }
}

impl Clone for wasi::http::types::Method {
    fn clone(&self) -> Self {
        use wasi::http::types::Method;
        match self {
            Method::Get => Method::Get,
            Method::Head => Method::Head,
            Method::Post => Method::Post,
            Method::Put => Method::Put,
            Method::Delete => Method::Delete,
            Method::Connect => Method::Connect,
            Method::Options => Method::Options,
            Method::Trace => Method::Trace,
            Method::Patch => Method::Patch,
            Method::Other(method) => Method::Other(method.clone()),
        }
    }
}

impl Clone for wasi::http::types::Scheme {
    fn clone(&self) -> Self {
        use wasi::http::types::Scheme;
        match self {
            Scheme::HTTP => Scheme::HTTP,
            Scheme::HTTPS => Scheme::HTTPS,
            Scheme::Other(scheme) => Scheme::Other(scheme.clone()),
        }
    }
}
