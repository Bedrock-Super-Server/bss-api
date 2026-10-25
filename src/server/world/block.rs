use crate::content::StateValue;
use crate::ffi::abi::FfiStr;
use crate::registry::server;
use crate::server::ffi_string;

/// A block state, such as stone or wheat at growth stage 7.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    runtime_id: u32,
}

impl Block {
    /**
    The default state of the block named `name`, such as `minecraft:stone`; air for an unknown name.

    # Panics

    Panics when called before the server enabled the plugin, such as from a `static` initializer.
    */
    pub fn named(name: &str) -> Block {
        Block {
            runtime_id: server().block_runtime_id(name),
        }
    }

    /// The block whose runtime id is `runtime_id`.
    pub fn from_runtime_id(runtime_id: u32) -> Block {
        Block { runtime_id }
    }

    /// Returns the runtime id of the block.
    pub fn runtime_id(self) -> u32 {
        self.runtime_id
    }

    /**
    Returns the name of the block, such as `minecraft:stone`.

    # Panics

    Panics when called before the server enabled the plugin.
    */
    pub fn name(self) -> String {
        server().block_name(self.runtime_id)
    }

    /**
    Returns the state `key` of the block as text, such as `top` for `minecraft:vertical_half`, or `None` when the block has
    no such state. A boolean reads as `0` or `1`.
    */
    pub fn state(self, key: &str) -> Option<String> {
        let value = host_call!(self.host(), block_state(self.runtime_id, FfiStr::new(key)))
            .map(ffi_string)?;
        (!value.is_empty()).then_some(value)
    }

    /// Returns the state `key` of the block as a whole number, or `None` when it has no such state or it is not a number.
    pub fn state_int(self, key: &str) -> Option<i32> {
        self.state(key)?.parse().ok()
    }

    /// Returns the state `key` of the block as a boolean, or `None` when it has no such state or it is not a boolean.
    pub fn state_bool(self, key: &str) -> Option<bool> {
        match self.state(key)?.as_str() {
            "0" => Some(false),
            "1" => Some(true),
            _ => None,
        }
    }

    /// Returns every state of the block as `(name, value)`, the value as text.
    pub fn states(self) -> Vec<(String, String)> {
        server().block_states(self.runtime_id)
    }

    /**
    Returns the block with its state `key` set to `value` and every other state as it was, or `None` when no block of this
    name has that state at that value.

    # Example

    ```no_run
    use bss_api::prelude::*;

    fn light(lamp: Block) -> Option<Block> {
        lamp.try_with("shop:lit", true)
    }
    ```
    */
    pub fn try_with(self, key: &str, value: impl Into<StateValue>) -> Option<Block> {
        let value = value.into().text();
        let runtime_id = host_call!(
            self.host(),
            block_with_state(self.runtime_id, FfiStr::new(key), FfiStr::new(&value))
        )?;
        (runtime_id != u32::MAX).then_some(Block { runtime_id })
    }

    /// Returns the block with its state `key` set to `value`, or the block itself when it has no such state or value.
    /// [`try_with`](Block::try_with) tells the two apart.
    pub fn with(self, key: &str, value: impl Into<StateValue>) -> Block {
        self.try_with(key, value).unwrap_or(self)
    }

    /// The block named `name` with each of `states` set, starting from its default state; a state it does not have or a
    /// value it does not take is left out.
    pub fn named_with(name: &str, states: &[(&str, StateValue)]) -> Block {
        states
            .iter()
            .fold(Block::named(name), |block, (key, value)| {
                block.with(key, value.clone())
            })
    }

    fn host(self) -> &'static crate::ffi::host::HostApi {
        server().host()
    }
}
