/*!
Why a call of the server did not do what was asked.

The methods that can fail return `Result<_, ApiError>` and say why: the plugin is not allowed to
make the call, the server is too old to have it, or what it names does not exist.

```
use bss_api::prelude::*;

fn ban_cheater(server: &Server) {
    match server.bans().ban("Cheater", None, "cheating") {
        Ok(()) => {}
        Err(ApiError::NeedsGrant(grant)) => {
            // Add `grants = ["moderation"]` to the plugin's section of plugins.toml.
            let _ = grant;
        }
        Err(other) => {
            let _ = other;
        }
    }
}
```
*/

use std::fmt;

/// Why a call of the server failed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ApiError {
    /// The server has no such function: it is older than this plugin expects.
    Unsupported,
    /// The plugin's section of `plugins.toml` denies the function.
    Denied,
    /// The function needs a grant the plugin does not have, one of [`GRANTS`](crate::ffi::GRANTS).
    NeedsGrant(&'static str),
    /// The player, world, entity or name the call names does not exist.
    NotFound,
    /// The server understood the call and refused it.
    Refused,
}

impl ApiError {
    /// Why the host function `function` is not in the table of the server: asks the server when it
    /// can tell, and says it is unsupported otherwise.
    pub(crate) fn absent(function: &str) -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            return match crate::ffi::wasm::refusal_of(function) {
                2 => ApiError::Denied,
                code if code >= 3 => crate::ffi::GRANTS
                    .get(code as usize - 3)
                    .map_or(ApiError::Denied, |grant| ApiError::NeedsGrant(grant)),
                _ => ApiError::Unsupported,
            };
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = function;
            ApiError::Unsupported
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Unsupported => f.write_str("the server is too old to have this function"),
            ApiError::Denied => {
                f.write_str("the plugin's section of plugins.toml denies this function")
            }
            ApiError::NeedsGrant(grant) => write!(
                f,
                "this function needs the grant \"{grant}\" in plugins.toml"
            ),
            ApiError::NotFound => f.write_str("what the call names does not exist"),
            ApiError::Refused => f.write_str("the server refused the call"),
        }
    }
}

impl std::error::Error for ApiError {}

/// What the methods that can fail return.
pub type ApiResult<T> = Result<T, ApiError>;
