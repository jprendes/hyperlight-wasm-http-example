use super::WasiImpl;
use crate::wasi_impl::bindings::wasi;

impl wasi::clocks::WallClock for WasiImpl {
    fn now(&mut self) -> wasi::clocks::wall_clock::Datetime {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        wasi::clocks::wall_clock::Datetime {
            seconds: now.as_secs(),
            nanoseconds: now.subsec_nanos(),
        }
    }

    fn resolution(&mut self) -> wasi::clocks::wall_clock::Datetime {
        wasi::clocks::wall_clock::Datetime {
            seconds: 1,
            nanoseconds: 0,
        }
    }
}
