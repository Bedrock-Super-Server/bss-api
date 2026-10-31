/*!
Declarations of the core events: each macro declares the [`Hook`](crate::registry::hook::Hook) of
an event and the `Server` method that registers a handler for it.
*/

/// An event handlers only observe.
macro_rules! event_hook {
    ($method:ident, $reg:ident, $entry:ident, $data:ty, $safe:ty, $build:expr) => {
        crate::registry::hook::hook!($entry, $data => $safe, $build);

        impl Server {
            pub(crate) fn $method(&self, handler: impl Fn(&Server, &$safe) + Send + Sync + 'static) {
                crate::registry::hook::register::<$entry, _>(move |server, event| {
                    handler(server, event);
                    false
                });
            }
        }
    };
}

/// An event handlers may cancel, and whose listed fields they may change.
macro_rules! event_hook_cancel {
    ($method:ident, $reg:ident, $entry:ident, $data:ty, $safe:ty, $build:expr) => {
        event_hook_cancel!($method, $reg, $entry, $data, $safe, $build, []);
    };
    ($method:ident, $reg:ident, $entry:ident, $data:ty, $safe:ty, $build:expr, [$($field:ident),*]) => {
        crate::registry::hook::hook!(
            $entry,
            $data => $safe,
            $build,
            |_safe: $safe, _raw: &mut $data| {
                $(_raw.$field = _safe.$field.get();)*
            }
        );

        impl crate::registry::hook::Cancellable for $safe {}

        impl $safe {
            /// Cancels the event: see its documentation for what that prevents.
            pub fn cancel(&self) {
                crate::registry::hook::Cancellable::set_cancelled(self, true);
            }
        }

        impl Server {
            pub(crate) fn $method(&self, handler: impl Fn(&Server, &$safe) -> bool + Send + Sync + 'static) {
                crate::registry::hook::register::<$entry, _>(handler);
            }
        }
    };
}

/// An event with a `name` and a `message` handlers may change.
macro_rules! event_hook_message {
    ($method:ident, $reg:ident, $entry:ident, $buffer:ident, $data:ty, $safe:ident) => {
        thread_local! {
            static $buffer: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
        }

        crate::registry::hook::hook!(
            $entry,
            $data => $safe,
            |raw: &$data| $safe {
                name: ffi_string(raw.name),
                message: std::cell::RefCell::new(ffi_string(raw.message)),
            },
            |safe: $safe, raw: &mut $data| {
                raw.message = crate::registry::hand_back(&$buffer, safe.message.into_inner());
            }
        );

        impl Server {
            pub(crate) fn $method(&self, handler: impl Fn(&Server, &$safe) + Send + Sync + 'static) {
                crate::registry::hook::register::<$entry, _>(move |server, event| {
                    handler(server, event);
                    false
                });
            }
        }
    };
}

/// An event handlers may cancel, with one text field they may change.
macro_rules! event_hook_cancel_message {
    ($method:ident, $reg:ident, $entry:ident, $buffer:ident, $data:ty, $safe:ty, $build:expr, $field:ident) => {
        thread_local! {
            static $buffer: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
        }

        crate::registry::hook::hook!(
            $entry,
            $data => $safe,
            $build,
            |safe: $safe, raw: &mut $data| {
                raw.$field = crate::registry::hand_back(&$buffer, safe.$field.into_inner());
            }
        );

        impl crate::registry::hook::Cancellable for $safe {}

        impl $safe {
            /// Cancels the event: see its documentation for what that prevents.
            pub fn cancel(&self) {
                crate::registry::hook::Cancellable::set_cancelled(self, true);
            }
        }

        impl Server {
            pub(crate) fn $method(&self, handler: impl Fn(&Server, &$safe) -> bool + Send + Sync + 'static) {
                crate::registry::hook::register::<$entry, _>(handler);
            }
        }
    };
}

pub(crate) use {event_hook, event_hook_cancel, event_hook_cancel_message, event_hook_message};
