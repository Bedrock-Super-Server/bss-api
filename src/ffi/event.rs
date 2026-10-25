/*!
How the server hands events to plugins.

A plugin subscribes to an event by its stable number with
[`HostApi::event_subscribe`](crate::ffi::HostApi::event_subscribe), at a priority, and the server
calls the [`EventCallback`] it gave with the event's data. Every event's data is a `#[repr(C)]`
struct that starts with an [`EventHeader`]: its size, its number, and whether it is cancelled.

The server calls the subscriptions of an event from the lowest priority to the highest, then the
[`MONITOR`] ones. Each sees what the previous ones did: whether they cancelled the event, and the
fields they changed. A subscription may cancel the event or take a cancellation back, except at
[`MONITOR`], where the server undoes whatever it changes.

Fields are only ever added at the end of an event's data. The `size` in its header says how many
bytes the server wrote: a plugin built for a newer minor version reads the fields it knows past
that size as zero, and one built for an older minor version simply does not see the newer ones.
*/

use core::ffi::c_void;

use crate::ffi::host::HostApi;

/// The start of every event's data. Its layout never changes.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EventHeader {
    /// Size in bytes of the event's data the server built, header included.
    pub size: u32,
    /// Stable number of the event, from [`EventData::ID`].
    pub id: u32,
    /// Nonzero once the event is cancelled. A subscription may set or clear it.
    pub cancelled: u8,
    /// Zero; room for flags of later minor versions.
    pub reserved: [u8; 7],
}

/// Called with an event a plugin subscribed to: the server's table, the event's data, which
/// starts with its [`EventHeader`], and the `user_data` the plugin subscribed with.
pub type EventCallback =
    unsafe extern "C" fn(host: *const HostApi, event: *mut EventHeader, user_data: *mut c_void);

/// Called first.
pub const LOWEST: i32 = -2000;
/// Called before [`NORMAL`].
pub const LOW: i32 = -1000;
/// The priority of most subscriptions.
pub const NORMAL: i32 = 0;
/// Called after [`NORMAL`].
pub const HIGH: i32 = 1000;
/// Called last among the subscriptions that may change the event: it has the final word.
pub const HIGHEST: i32 = 2000;
/// Called after every other subscription, to observe the outcome. What it changes is undone.
pub const MONITOR: i32 = i32::MAX;

/// Subscription flag: do not call this subscription for an event already cancelled.
pub const IGNORE_CANCELLED: u32 = 1;

/**
The data of an event: a `#[repr(C)]` struct whose first field is an [`EventHeader`] named `header`.

# Safety

The type must be `#[repr(C)]`, start with an [`EventHeader`], accept all-zero bytes for every
field, and only ever grow by fields added at its end. [`event_data!`](crate::event_data) upholds
the first two for the types it is given.
*/
pub unsafe trait EventData: Sized + 'static {
    /// Stable number of the event. Numbers are never reused.
    const ID: u32;
    /// Name of the event, as in `on_<name>`.
    const NAME: &'static str;

    /// The header of the event.
    fn header(&self) -> &EventHeader;

    /// The header of the event, to cancel it or take the cancellation back.
    fn header_mut(&mut self) -> &mut EventHeader;

    /// A header for this event, as the server fills it before calling subscriptions.
    fn new_header() -> EventHeader {
        EventHeader {
            size: core::mem::size_of::<Self>() as u32,
            id: Self::ID,
            cancelled: 0,
            reserved: [0; 7],
        }
    }
}

/// Declares types as event data with their stable numbers and names:
/// `event_data!(PlayerChatEventData = 4, "chat");`.
#[doc(hidden)]
#[macro_export]
macro_rules! event_data {
    ($($data:ty = $id:literal, $name:literal;)*) => {
        $(
            // SAFETY: the type is `#[repr(C)]`, starts with `header: EventHeader`, and is made of
            // integers, floats, booleans and `FfiStr`, which all accept zero.
            unsafe impl $crate::ffi::event::EventData for $data {
                const ID: u32 = $id;
                const NAME: &'static str = $name;

                fn header(&self) -> &$crate::ffi::event::EventHeader {
                    &self.header
                }

                fn header_mut(&mut self) -> &mut $crate::ffi::event::EventHeader {
                    &mut self.header
                }
            }

            const _: () = assert!(::core::mem::offset_of!($data, header) == 0);
        )*
    };
}
