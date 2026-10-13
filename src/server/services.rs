use core::ffi::c_void;
use parking_lot::Mutex;
use std::cell::RefCell;
use std::panic::{AssertUnwindSafe, catch_unwind};

use crate::ffi::abi::{FfiStr, ServiceCallback};
use crate::ffi::call::HostFn;
use crate::server::{Server, ffi_string};

type Handler = Box<dyn Fn(&str) -> String + Send + Sync>;

static HANDLERS: Mutex<Vec<&'static Handler>> = Mutex::new(Vec::new());

thread_local! {
    static RESPONSE: RefCell<String> = const { RefCell::new(String::new()) };
}

unsafe extern "C" fn answer_request(request: FfiStr, user_data: *mut c_void) -> FfiStr {
    // SAFETY: `user_data` is the `&'static Handler` that `register_service` leaked
    // for this service; it is never freed.
    let handler = unsafe { &*(user_data as *const Handler) };
    let req = ffi_string(request);
    let resp = catch_unwind(AssertUnwindSafe(|| handler(&req))).unwrap_or_default();
    RESPONSE.with(|cell| {
        let mut slot = cell.borrow_mut();
        *slot = resp;
        FfiStr {
            ptr: slot.as_ptr(),
            len: slot.len(),
        }
    })
}

impl Server {
    /**
    Offers a service to other plugins under `name`, replacing any service of the same name. `handler` receives a request and returns the answer; both are free text, usually JSON.

    ```
    use bss_api::prelude::*;

    // In an economy plugin:
    fn offer_balance(server: &Server) {
        server.register_service("economy.balance", |player| format!("{{\"player\":\"{player}\",\"coins\":120}}"));
    }

    // In another plugin:
    fn coins(server: &Server, player: &str) -> Option<String> {
        server.call_service("economy.balance", player)
    }
    ```
    */
    pub fn register_service(
        &self,
        name: &str,
        handler: impl Fn(&str) -> String + Send + Sync + 'static,
    ) {
        let boxed: Handler = Box::new(handler);
        let leaked: &'static Handler = Box::leak(Box::new(boxed));
        HANDLERS.lock().push(leaked);
        host_call!(
            self.host(),
            service_register(
                FfiStr::new(name),
                ServiceCallback::Some(answer_request),
                leaked as *const Handler as *mut c_void
            )
        );
    }

    /// Calls the service `name` with `request` and returns its answer. Returns `None` when this server cannot call services; an unknown service answers with an empty string.
    pub fn call_service(&self, name: &str, request: &str) -> Option<String> {
        let f = self.host().service_call?;
        let response = ffi_string(f.call((FfiStr::new(name), FfiStr::new(request))));
        Some(response)
    }
}
