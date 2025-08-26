use std::time::Duration;

use hyperlight_common::resource::BorrowedResourceGuard;

use super::WasiImpl;

use crate::{
    bindings::wasi,
    resource::{BlockOn, Resource},
};

#[derive(Default)]
pub struct RequestOptions {
    connect_timeout: Option<Duration>,
    first_byte_timeout: Option<Duration>,
    between_bytes_timeout: Option<Duration>,
}

fn as_u64_nanos_saturating(duration: &Duration) -> u64 {
    duration.as_nanos().min(u64::MAX as u128) as u64
}

impl wasi::http::types::RequestOptions<u64> for WasiImpl {
    type T = Resource<RequestOptions>;

    fn new(&mut self) -> Self::T {
        Resource::default()
    }

    fn connect_timeout(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Option<u64> {
        self_
            .read()
            .block_on()
            .connect_timeout
            .as_ref()
            .map(as_u64_nanos_saturating)
    }

    fn set_connect_timeout(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        duration: Option<u64>,
    ) -> Result<(), ()> {
        self_.write().block_on().connect_timeout = duration.map(Duration::from_nanos);
        Ok(())
    }

    fn first_byte_timeout(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Option<u64> {
        self_
            .read()
            .block_on()
            .first_byte_timeout
            .as_ref()
            .map(as_u64_nanos_saturating)
    }

    fn set_first_byte_timeout(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        duration: Option<u64>,
    ) -> ::core::result::Result<(), ()> {
        self_.write().block_on().first_byte_timeout = duration.map(Duration::from_nanos);
        Ok(())
    }

    fn between_bytes_timeout(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Option<u64> {
        self_
            .read()
            .block_on()
            .between_bytes_timeout
            .as_ref()
            .map(as_u64_nanos_saturating)
    }

    fn set_between_bytes_timeout(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        duration: Option<u64>,
    ) -> ::core::result::Result<(), ()> {
        self_.write().block_on().between_bytes_timeout = duration.map(Duration::from_nanos);
        Ok(())
    }
}
