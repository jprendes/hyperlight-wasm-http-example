use crate::{
    bindings::wasi,
    resource::{BlockOn as _, Resource},
};

use super::{WasiImpl, http_outgoing_response::OutgoingResponse};

#[derive(Default)]
pub struct ResponseOutparam {
    pub response: Option<Result<Resource<OutgoingResponse>, wasi::http::types::ErrorCode>>,
}

impl wasi::http::types::ResponseOutparam<Resource<OutgoingResponse>> for WasiImpl {
    type T = Resource<ResponseOutparam>;

    fn set(
        &mut self,
        param: Self::T,
        response: Result<Resource<OutgoingResponse>, wasi::http::types::ErrorCode>,
    ) -> () {
        param.write().block_on().response = Some(response);
    }
}
