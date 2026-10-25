/*!
Access to the memory the server passes to a callback.

The server calls into a plugin through `extern "C"` callbacks that receive raw pointers: the [`HostApi`](crate::ffi::HostApi) table and the event being dispatched. These two functions hold the only dereference of those pointers.
*/

/**
Borrows a value the server passed to the callback that calls this function.

# Contract for callers in this crate

`pointer` must be an argument the server passed to the calling callback. The server always passes a non-null pointer to a value that lives on its side for the whole call and that nothing else touches until the call returns, so the borrow must not outlive the callback.

# Panics

Panics if `pointer` is null, which the server never does. Since a panic cannot unwind out of an `extern "C"` function, the process would abort.
*/
pub(crate) fn host_ref<'a, T>(pointer: *const T) -> &'a T {
    assert!(
        !pointer.is_null(),
        "the server passed a null pointer to a plugin"
    );
    // SAFETY: non-null, and by the contract above it points at a live value that
    // nothing else mutates while the callback runs.
    unsafe { &*pointer }
}

/**
Copies a byte buffer the server passed to a callback. A null pointer reads as empty.

The same contract as [`host_ref`] applies to `pointer` and `len`.
*/
pub(crate) fn host_bytes(pointer: *const u8, len: usize) -> Vec<u8> {
    if pointer.is_null() {
        return Vec::new();
    }
    // SAFETY: by the contract above, `pointer` points at `len` readable bytes
    // for the duration of the call; they are copied before returning.
    unsafe { std::slice::from_raw_parts(pointer, len) }.to_vec()
}
