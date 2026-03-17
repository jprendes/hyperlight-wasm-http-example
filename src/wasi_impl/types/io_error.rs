use hyperlight_common::resource::BorrowedResourceGuard;

use crate::wasi_impl::types::WasiImpl;

use crate::wasi_impl::bindings::wasi;

impl wasi::io::error::Error for WasiImpl {
    type T = anyhow::Error;

    fn to_debug_string(&mut self, self_: BorrowedResourceGuard<Self::T>) -> String {
        self_.to_string()
    }
}

impl wasi::io::Error for WasiImpl {}
