use crate::ffi::abi::ChunkHandle;
use crate::ffi::host::HostApi;
use crate::server::world::Block;

/// A chunk being generated, lent to a generator registered with [`Server::register_generator`](crate::server::Server::register_generator).
pub struct Chunk {
    host: &'static HostApi,
    handle: ChunkHandle,
}

impl Chunk {
    pub(crate) fn new(host: &'static HostApi, handle: ChunkHandle) -> Self {
        Self { host, handle }
    }

    /// Places `block` at `(x, y, z)`, where `x` and `z` go from `0` to `15` inside the chunk and `y` is the world height. Coordinates outside the chunk are ignored.
    pub fn set(&mut self, x: i32, y: i32, z: i32, block: Block) {
        host_call!(
            self.host,
            chunk_set_block(self.handle, x, y, z, block.runtime_id())
        );
    }
}
