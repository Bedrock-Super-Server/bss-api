use parking_lot::Mutex;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::ffi::host::HostApi;
use crate::registry::{server, set_host};
use crate::server::Server;

pub(crate) type Handlers<E> = Mutex<Vec<&'static (dyn Fn(&Server, &E) -> bool + Send + Sync)>>;

pub(crate) fn register<E>(
    handlers: &Handlers<E>,
    handler: impl Fn(&Server, &E) -> bool + Send + Sync + 'static,
) {
    handlers.lock().push(Box::leak(Box::new(handler)));
}

pub(crate) fn run<E>(
    host: *const HostApi,
    handlers: &Handlers<E>,
    build: impl FnOnce() -> E,
    finish: impl FnOnce(E),
) -> bool {
    set_host(host);
    let handlers: Vec<_> = {
        let registered = handlers.lock();
        if registered.is_empty() {
            return false;
        }
        registered.clone()
    };
    let server = server();
    let event = build();
    let mut cancelled = false;
    for handler in handlers {
        if let Ok(true) = catch_unwind(AssertUnwindSafe(|| handler(&server, &event))) {
            cancelled = true;
        }
    }
    finish(event);
    cancelled
}
