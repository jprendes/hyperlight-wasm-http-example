use std::{sync::LazyLock, time::Duration};

use crate::{bindings::wasi, resource::Resource};

use super::{WasiImpl, io_poll::AnyPollable};

static EPOCH: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);

fn now() -> u64 {
    let now = std::time::Instant::now().duration_since(*EPOCH);
    now.as_nanos() as u64
}

impl wasi::clocks::MonotonicClock<Resource<AnyPollable>> for WasiImpl {
    fn now(&mut self) -> u64 {
        now()
    }

    fn resolution(&mut self) -> u64 {
        1
    }

    fn subscribe_instant(&mut self, when: u64) -> Resource<AnyPollable> {
        Resource::new(AnyPollable::future(tokio::time::sleep_until(
            tokio::time::Instant::now() + Duration::from_nanos(when - now()),
        )))
    }

    fn subscribe_duration(&mut self, when: u64) -> Resource<AnyPollable> {
        Resource::new(AnyPollable::future(tokio::time::sleep(
            Duration::from_nanos(when),
        )))
    }
}
