/*!
How the SDK turns the server's events into calls of the plugin's handlers.

Each kind of event has a [`Hook`]: the C layout of its data, the event the handlers see, and how
to read one from the other and write the changes back. The handlers registered with the same
[`EventOptions`] share one subscription to the server, made when the first of them is registered.
*/

use core::ffi::c_void;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};

use parking_lot::Mutex;

use crate::ffi::event::{self, EventData, EventHeader};
use crate::ffi::host::HostApi;
use crate::ffi::version::copy_sized;
use crate::registry::{server, set_host};
use crate::server::Server;

/**
What a closure given to a `Server::on_*` method answers: nothing, or a `bool`.

A closure cancels the event by calling [`Cancellable::cancel`] on it, or by returning `true`.

```
use bss_api::prelude::*;

fn register(server: &Server) {
    // Nothing is returned: the closure cancels the event itself.
    server.on_tree_grow(|_server, event| {
        if event.y > 100 {
            event.cancel();
        }
    });

    // Returning `true` cancels it too.
    server.on_block_fall(|_server, event| event.y < 0);
}
```
*/
pub trait Verdict {
    /// Whether the handler cancels the event.
    fn cancels(self) -> bool;
}

impl Verdict for () {
    fn cancels(self) -> bool {
        false
    }
}

impl Verdict for bool {
    fn cancels(self) -> bool {
        self
    }
}

/// When a handler runs among the handlers of every plugin for the same event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Priority {
    /// Runs first.
    Lowest,
    /// Runs before [`Normal`](Priority::Normal).
    Low,
    /// The priority of most handlers.
    #[default]
    Normal,
    /// Runs after [`Normal`](Priority::Normal).
    High,
    /// Runs last among the handlers that may change the event: it has the final word.
    Highest,
    /// Runs after every other handler, to observe the outcome. What it changes, cancelling
    /// included, is undone.
    Monitor,
}

impl Priority {
    /// The number the server orders subscriptions by.
    pub const fn value(self) -> i32 {
        match self {
            Self::Lowest => event::LOWEST,
            Self::Low => event::LOW,
            Self::Normal => event::NORMAL,
            Self::High => event::HIGH,
            Self::Highest => event::HIGHEST,
            Self::Monitor => event::MONITOR,
        }
    }
}

/**
How handlers are registered: their [`Priority`] and whether they skip cancelled events.

```
use bss_api::prelude::*;

struct Protect;

impl Event for Protect {
    fn on_block_break(&self, _server: &Server, event: &BlockBreakEvent) {
        event.cancel();
    }
}

struct Log;

impl Plugin for Log {
    fn register(&self, server: &Server) {
        server.register_event_with(Protect, EventOptions::at(Priority::Low));
        server.with_event_options(EventOptions::at(Priority::Monitor).ignore_cancelled(), |server| {
            server.on_tree_grow(|_, event| {
                println!("a tree grew at {} {} {}", event.x, event.y, event.z);
                false
            });
        });
    }
}
```
*/
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EventOptions {
    /// When the handlers run.
    pub priority: Priority,
    /// Whether they skip an event another handler already cancelled.
    pub ignore_cancelled: bool,
}

impl EventOptions {
    /// Handlers at `priority`, called for cancelled events too.
    pub const fn at(priority: Priority) -> Self {
        Self {
            priority,
            ignore_cancelled: false,
        }
    }

    /// The same options, skipping events already cancelled.
    pub const fn ignore_cancelled(mut self) -> Self {
        self.ignore_cancelled = true;
        self
    }

    fn flags(self) -> u32 {
        if self.ignore_cancelled {
            event::IGNORE_CANCELLED
        } else {
            0
        }
    }
}

thread_local! {
    /// The options `register` gives new handlers, set by [`Server::with_event_options`].
    static OPTIONS: Cell<EventOptions> = const {
        Cell::new(EventOptions {
            priority: Priority::Normal,
            ignore_cancelled: false,
        })
    };

    /// Whether the event the running handler sees is cancelled.
    static CANCELLED: Cell<bool> = const { Cell::new(false) };
}

impl Server {
    /**
    Registers the handlers `register` registers with `options` instead of the default ones: the
    `on_*` methods, [`register_event`](Server::register_event) included.
    */
    pub fn with_event_options<R>(
        &self,
        options: EventOptions,
        register: impl FnOnce(&Server) -> R,
    ) -> R {
        let previous = OPTIONS.replace(options);
        struct Restore(EventOptions);
        impl Drop for Restore {
            fn drop(&mut self) {
                OPTIONS.set(self.0);
            }
        }
        let _restore = Restore(previous);
        register(self)
    }
}

/// The options handlers registered now get.
pub(crate) fn current_options() -> EventOptions {
    OPTIONS.get()
}

/**
An event that can be cancelled: what the handlers before this one decided, and the way to change it.

Only meaningful while a handler of the event runs. `event.cancel()` cancels the
event too; returning `false` leaves it as it is.
*/
pub trait Cancellable {
    /// Whether the event is cancelled at this point: by a handler of a lower priority, or by an
    /// earlier handler of this one.
    fn is_cancelled(&self) -> bool {
        CANCELLED.get()
    }

    /// Cancels the event, or takes the cancellation back with `false`.
    fn set_cancelled(&self, cancelled: bool) {
        CANCELLED.set(cancelled);
    }

    /// Cancels the event.
    fn cancel(&self) {
        self.set_cancelled(true);
    }
}

/// Runs a handler that sees the event as `cancelled`, and returns whether the event is cancelled
/// after it.
pub(crate) fn run_handler(cancelled: bool, handler: impl FnOnce() -> bool) -> bool {
    let outer = CANCELLED.replace(cancelled);
    if catch_unwind(AssertUnwindSafe(handler)).unwrap_or(false) {
        CANCELLED.set(true);
    }
    CANCELLED.replace(outer)
}

/// Whether the running event is cancelled.
pub(crate) fn is_cancelled() -> bool {
    CANCELLED.get()
}

/// A handler as the SDK keeps it.
pub(crate) type Handler<S> = &'static (dyn Fn(&Server, &S) -> bool + Send + Sync);

/// The handlers of one kind of event that share options, and so one subscription.
pub(crate) struct Bucket<S: 'static> {
    options: EventOptions,
    handlers: Mutex<Vec<Handler<S>>>,
}

/// Every [`Bucket`] of one kind of event.
pub(crate) type Buckets<S> = Mutex<Vec<&'static Bucket<S>>>;

/// A kind of event, as the SDK reads it and writes it back.
pub(crate) trait Hook: 'static {
    /// The C layout of the event.
    type Data: EventData;
    /// What the handlers see.
    type Safe: 'static;

    /// The buckets of this event.
    fn buckets() -> &'static Buckets<Self::Safe>;

    /// Reads what the handlers see.
    fn build(raw: &Self::Data) -> Self::Safe;

    /// Writes the fields the handlers may change back into the event.
    fn finish(safe: Self::Safe, raw: &mut Self::Data) {
        let _ = (safe, raw);
    }
}

/// Registers `handler` for the event of `H` with the [current options](current_options).
pub(crate) fn register<H: Hook, V: Verdict>(
    handler: impl Fn(&Server, &H::Safe) -> V + Send + Sync + 'static,
) {
    let options = current_options();
    let handler: Handler<H::Safe> = Box::leak(Box::new(move |server: &Server, event: &H::Safe| {
        handler(server, event).cancels()
    }));
    let mut buckets = H::buckets().lock();
    if let Some(bucket) = buckets.iter().find(|bucket| bucket.options == options) {
        bucket.handlers.lock().push(handler);
        return;
    }
    let bucket: &'static Bucket<H::Safe> = Box::leak(Box::new(Bucket {
        options,
        handlers: Mutex::new(vec![handler]),
    }));
    buckets.push(bucket);
    drop(buckets);
    let user_data = bucket as *const Bucket<H::Safe> as *mut c_void;
    let callback: event::EventCallback = entry::<H>;
    host_call!(
        server().host(),
        event_subscribe(
            H::Data::ID,
            options.priority.value(),
            options.flags(),
            callback,
            user_data
        )
    );
}

/// What the server calls for an event of `H`, with the bucket the subscription was made for. The
/// handlers already catch their own panics; this also catches one while the event is read or
/// written back, which would otherwise unwind out of an `extern "C"` function and abort the server.
unsafe extern "C" fn entry<H: Hook>(
    host: *const HostApi,
    header: *mut EventHeader,
    user_data: *mut c_void,
) {
    // SAFETY: the arguments are the server's, passed on unchanged.
    let delivered = catch_unwind(AssertUnwindSafe(|| unsafe {
        deliver::<H>(host, header, user_data)
    }));
    if delivered.is_err() {
        eprintln!(
            "[bss-api] a panic while an event was read or written back; the event is left as the server sent it"
        );
    }
}

/// The body of [`entry`], with the same contract.
unsafe fn deliver<H: Hook>(host: *const HostApi, header: *mut EventHeader, user_data: *mut c_void) {
    if header.is_null() || user_data.is_null() || !set_host(host) {
        return;
    }
    // SAFETY: `user_data` is the leaked bucket `register` subscribed with, for this `H`.
    let bucket = unsafe { &*(user_data as *const Bucket<H::Safe>) };
    let handlers: Vec<Handler<H::Safe>> = bucket.handlers.lock().clone();
    if handlers.is_empty() {
        return;
    }
    // SAFETY: the server passes the data of the event `H` subscribed to, which starts with its
    // header; `size` is how many bytes of it the server built.
    let size = unsafe { (*header).size } as usize;
    let full = size >= std::mem::size_of::<H::Data>();
    // A server of an older minor version built a shorter event: the fields it lacks are read from
    // a copy where they are zero, and only the bytes it built are written back.
    // SAFETY: `header` points at `size` bytes, and `H::Data` accepts zero.
    let mut copy = (!full).then(|| unsafe { copy_sized::<H::Data>(header.cast(), size) });
    let raw: &mut H::Data = match copy.as_mut() {
        Some(copy) => copy,
        // SAFETY: the server's data holds at least a whole `H::Data`, which is its prefix, and
        // nothing else touches it during the call.
        None => unsafe { &mut *header.cast::<H::Data>() },
    };
    let safe = H::build(raw);
    let server = server();
    let mut cancelled = raw.header().cancelled != 0;
    for handler in handlers {
        if cancelled && bucket.options.ignore_cancelled {
            break;
        }
        cancelled = run_handler(cancelled, || handler(&server, &safe));
    }
    H::finish(safe, raw);
    raw.header_mut().cancelled = cancelled as u8;
    if let Some(copy) = copy {
        // SAFETY: writes back the `size` bytes the server built, from a copy of them.
        unsafe {
            std::ptr::copy_nonoverlapping(
                (&copy as *const H::Data).cast::<u8>(),
                header.cast::<u8>(),
                size,
            )
        };
    }
}

/// Declares the [`Hook`] of an event: `hook!(ChatHook, PlayerChatEventData => ChatEvent, build, finish);`.
macro_rules! hook {
    ($hook:ident, $data:ty => $safe:ty, $build:expr) => {
        $crate::registry::hook::hook!($hook, $data => $safe, $build, |_safe: $safe, _raw: &mut $data| {});
    };
    ($hook:ident, $data:ty => $safe:ty, $build:expr, $finish:expr) => {
        #[allow(non_camel_case_types)]
        pub(crate) struct $hook;

        impl $crate::registry::hook::Hook for $hook {
            type Data = $data;
            type Safe = $safe;

            fn buckets() -> &'static $crate::registry::hook::Buckets<$safe> {
                static BUCKETS: $crate::registry::hook::Buckets<$safe> =
                    ::parking_lot::Mutex::new(Vec::new());
                &BUCKETS
            }

            fn build(raw: &$data) -> $safe {
                ($build)(raw)
            }

            fn finish(safe: $safe, raw: &mut $data) {
                ($finish)(safe, raw)
            }
        }
    };
}

pub(crate) use hook;
