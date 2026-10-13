/*!
The one place where this crate calls into the server.

Every function of [`HostApi`](super::HostApi) is an `unsafe extern "C" fn`. Rather than wrapping each of the hundreds of calls in its own `unsafe` block, the crate calls them through [`HostFn::call`], whose body holds the only `unsafe` block, and through the [`host_call!`] macro built on it.
*/

/**
A function pointer from [`HostApi`](super::HostApi), callable with its arguments packed in a tuple.

# Contract for callers in this crate

The arguments are evaluated before the call, outside any `unsafe` block, so they cannot hide an unsafe operation. What remains is the contract documented on [`HostApi`](super::HostApi): every pointer passed, whether a [`FfiStr`](super::FfiStr) or a `(pointer, length)` pair, must point at memory that stays valid until the call returns, and an output buffer must be passed with its real capacity. Within this crate, arguments are always built from a live borrow at the call site, which satisfies it.
*/
pub(crate) trait HostFn<Args> {
    /// What the host function returns.
    type Output;

    /// Calls the function with `args`.
    fn call(self, args: Args) -> Self::Output;
}

macro_rules! impl_host_fn {
    ($($arg:ident),*) => {
        impl<R, $($arg),*> HostFn<($($arg,)*)> for unsafe extern "C" fn($($arg),*) -> R {
            type Output = R;

            #[allow(non_snake_case)]
            #[inline(always)]
            fn call(self, ($($arg,)*): ($($arg,)*)) -> R {
                // SAFETY: the server fills `HostApi` with functions that accept any
                // argument value and only read or write through pointers during the
                // call; callers in this crate only pass pointers to live buffers with
                // their real lengths (see the trait documentation).
                unsafe { self($($arg),*) }
            }
        }
    };
}

impl_host_fn!();
impl_host_fn!(A);
impl_host_fn!(A, B);
impl_host_fn!(A, B, C);
impl_host_fn!(A, B, C, D);
impl_host_fn!(A, B, C, D, E);
impl_host_fn!(A, B, C, D, E, F);
impl_host_fn!(A, B, C, D, E, F, G);
impl_host_fn!(A, B, C, D, E, F, G, H);
impl_host_fn!(A, B, C, D, E, F, G, H, I);
impl_host_fn!(A, B, C, D, E, F, G, H, I, J);
impl_host_fn!(A, B, C, D, E, F, G, H, I, J, K);
impl_host_fn!(A, B, C, D, E, F, G, H, I, J, K, L);

/**
Calls a function of the host table, if the server provides it.

`host_call!(host, name(a, b))` evaluates to `Some(result)` when `host.name` is set and `None` otherwise, which lets an older server that lacks a newer function answer with a neutral value instead of crashing the plugin:

```ignore
let count = host_call!(self.host, player_count()).unwrap_or(0);
```
*/
macro_rules! host_call {
    ($host:expr, $name:ident($($arg:expr),* $(,)?)) => {{
        let args = ($($arg,)*);
        $host.$name.map(|function| $crate::ffi::call::HostFn::call(function, args))
    }};
}
