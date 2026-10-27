/*!
The version of the plugin ABI and the headers that carry it.

The ABI is the set of `#[repr(C)]` tables and functions the server and a plugin share. Its version
has two parts:

- [`ABI_MAJOR`] changes only when something existing changes or goes away. A plugin and a server
  of different majors refuse each other.
- [`ABI_MINOR`] grows each time something is added: a function at the end of
  [`HostApi`](crate::ffi::HostApi), an event, a field at the end of an event. A plugin built for an
  older minor runs unchanged on a newer server, and one built for a newer minor still loads on an
  older server, where what was added since reads as absent.

Every table that crosses the boundary starts with a header that gives its size, so that each side
only reads what the other one wrote. Those headers never change.
*/

/// Changes when the ABI breaks: something existing changed or was removed.
pub const ABI_MAJOR: u16 = 1;

/// Grows with every addition to the ABI.
pub const ABI_MINOR: u16 = 12;

/// The value [`HostHeader::magic`] holds: `BSSHOST1` in ASCII.
pub const HOST_MAGIC: u64 = u64::from_be_bytes(*b"BSSHOST1");

/// The start of [`HostApi`](crate::ffi::HostApi). Its layout never changes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostHeader {
    /// Always [`HOST_MAGIC`].
    pub magic: u64,
    /// Size in bytes of the table the server built, header included. Functions past it are absent.
    pub size: u32,
    /// [`ABI_MAJOR`] of the server.
    pub major: u16,
    /// [`ABI_MINOR`] of the server.
    pub minor: u16,
}

impl HostHeader {
    /// The header of a table of type `T` built against this crate.
    pub const fn of<T>() -> Self {
        Self {
            magic: HOST_MAGIC,
            size: core::mem::size_of::<T>() as u32,
            major: ABI_MAJOR,
            minor: ABI_MINOR,
        }
    }
}

/// The start of the table a plugin describes itself with. Its layout never changes.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PluginHeader {
    /// Size in bytes of the table the plugin built, header included. Fields past it read as zero.
    pub size: u32,
    /// [`ABI_MAJOR`] the plugin was built with.
    pub major: u16,
    /// [`ABI_MINOR`] the plugin was built with.
    pub minor: u16,
}

impl PluginHeader {
    /// The header of a table of type `T` built against this crate.
    pub const fn of<T>() -> Self {
        Self {
            size: core::mem::size_of::<T>() as u32,
            major: ABI_MAJOR,
            minor: ABI_MINOR,
        }
    }
}

/**
Copies a table that starts with a size into a value of `T`, zero past the bytes the other side
wrote, and ignoring the bytes it wrote that this side does not know.

# Safety

`source` must point at `size` readable bytes, where `size` is what the table's header says, and
every field of `T` must accept all-zero bytes: `Option` of a function pointer, integers, floats,
raw pointers and the structs made of them.
*/
pub unsafe fn copy_sized<T>(source: *const u8, size: usize) -> T {
    let mut value = core::mem::MaybeUninit::<T>::zeroed();
    let count = size.min(core::mem::size_of::<T>());
    // SAFETY: by the contract above `source` holds `size` bytes, and `value` has room for
    // `size_of::<T>()`; all-zero bytes, then the bytes copied over them, are a valid `T`.
    unsafe {
        core::ptr::copy_nonoverlapping(source, value.as_mut_ptr().cast::<u8>(), count);
        value.assume_init()
    }
}
