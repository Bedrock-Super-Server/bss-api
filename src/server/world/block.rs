use crate::registry::server;

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
}
