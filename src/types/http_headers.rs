use hyperlight_common::resource::BorrowedResourceGuard;

use super::{
    WasiImpl,
    headers::{HeaderError, Headers},
};
use crate::{
    bindings::wasi,
    resource::{BlockOn, Resource},
};
use wasi::http::types::{FieldName, FieldValue};

impl From<HeaderError> for wasi::http::types::HeaderError {
    fn from(err: HeaderError) -> Self {
        match err {
            HeaderError::Immutable => wasi::http::types::HeaderError::Immutable,
            HeaderError::InvalidHeader => wasi::http::types::HeaderError::InvalidSyntax,
        }
    }
}

impl wasi::http::types::Fields for WasiImpl {
    type T = Resource<Headers>;

    fn new(&mut self) -> Self::T {
        Resource::default()
    }

    fn from_list(
        &mut self,
        entries: Vec<(FieldName, FieldValue)>,
    ) -> Result<Self::T, wasi::http::types::HeaderError> {
        Ok(Resource::new(Headers::from_list(entries)?))
    }

    fn get(&mut self, self_: BorrowedResourceGuard<Self::T>, name: FieldName) -> Vec<FieldValue> {
        self_.read().block_on().get(name).unwrap_or_default()
    }

    fn has(&mut self, self_: BorrowedResourceGuard<Self::T>, name: FieldName) -> bool {
        self_.read().block_on().has(name)
    }

    fn set(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        name: FieldName,
        value: Vec<FieldValue>,
    ) -> Result<(), wasi::http::types::HeaderError> {
        Ok(self_.write().block_on().set(name, value)?)
    }

    fn delete(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        name: FieldName,
    ) -> Result<(), wasi::http::types::HeaderError> {
        Ok(self_.write().block_on().delete(name)?)
    }

    fn append(
        &mut self,
        self_: BorrowedResourceGuard<Self::T>,
        name: FieldName,
        value: FieldValue,
    ) -> Result<(), wasi::http::types::HeaderError> {
        Ok(self_.write().block_on().append(name, value)?)
    }

    fn entries(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Vec<(FieldName, FieldValue)> {
        self_.read().block_on().entries()
    }

    fn clone(&mut self, self_: BorrowedResourceGuard<Self::T>) -> Self::T {
        self_.clone()
    }
}
