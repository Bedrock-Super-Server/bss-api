/**
A block receives a random tick, which drives slow changes such as crop growth or leaf decay.

Returning `true` from a handler skips this tick.

Handled by [`Event::on_random_tick`](crate::prelude::Event::on_random_tick).
*/
pub struct RandomTickEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block; [`Server::block_name`](crate::server::Server::block_name) gives its name.
    pub runtime_id: u32,
}
