use std::sync::LazyLock;

use tokio::runtime::Runtime;

pub static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(1)
        .build()
        .unwrap()
});
