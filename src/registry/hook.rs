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
    fn on_block_break(&self, _server: &Server, _event: &BlockBreakEvent) -> bool {
        true
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

Only meaningful while a handler of the event runs. Returning `true` from a handler cancels the
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

/// Marks the running event cancelled, for handlers that dispatch to several others.
pub(crate) fn cancel_current() {
    CANCELLED.set(true);
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
pub(crate) fn register<H: Hook>(
    handler: impl Fn(&Server, &H::Safe) -> bool + Send + Sync + 'static,
) {
    let options = current_options();
    let handler: Handler<H::Safe> = Box::leak(Box::new(handler));
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

/// What the server calls for an event of `H`, with the bucket the subscription was made for.
unsafe extern "C" fn entry<H: Hook>(
    host: *const HostApi,
    header: *mut EventHeader,
    user_data: *mut c_void,
) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::version::HostHeader;

    /// An event as a newer bss-api knows it: `added` came in a later minor version.
    #[repr(C)]
    pub(crate) struct NewerData {
        header: EventHeader,
        kept: u32,
        added: u32,
    }

    crate::event_data! {
        NewerData = 1999, "newer";
    }

    pub(crate) struct Seen {
        kept: Cell<u32>,
        added: u32,
    }

    hook!(
        newer,
        NewerData => Seen,
        |raw: &NewerData| Seen {
            kept: Cell::new(raw.kept),
            added: raw.added,
        },
        |safe: Seen, raw: &mut NewerData| {
            raw.kept = safe.kept.get();
            raw.added = 99;
        }
    );

    fn host() -> &'static HostApi {
        // SAFETY: every field of `HostApi` accepts zero; the table only needs its header here.
        let mut host: HostApi = unsafe { std::mem::zeroed() };
        host.header = HostHeader::of::<HostApi>();
        Box::leak(Box::new(host))
    }

    #[test]
    fn an_event_from_an_older_server_reads_as_zero_past_its_end() {
        let host = host();
        assert!(set_host(host));
        static ADDED: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(u32::MAX);
        register::<newer>(|_, seen| {
            ADDED.store(seen.added, std::sync::atomic::Ordering::Relaxed);
            seen.kept.set(seen.kept.get() + 1);
            true
        });
        let bucket = newer::buckets().lock()[0] as *const Bucket<Seen> as *mut c_void;

        // An older server built the event without `added`; what follows it in memory is not its.
        let mut memory = NewerData {
            header: EventHeader {
                size: std::mem::offset_of!(NewerData, added) as u32,
                id: 1999,
                ..EventHeader::default()
            },
            kept: 41,
            added: 7,
        };
        // SAFETY: `memory` holds the event the header describes, and `bucket` is the one
        // `register` made for `newer`.
        unsafe { entry::<newer>(host, (&mut memory as *mut NewerData).cast(), bucket) };
        assert_eq!(
            ADDED.load(std::sync::atomic::Ordering::Relaxed),
            0,
            "read as zero"
        );
        assert_eq!(
            memory.kept, 42,
            "the change to a known field is written back"
        );
        assert_eq!(memory.added, 7, "nothing is written past the event");
        assert_eq!(memory.header.cancelled, 1);
    }
}
