/**
A block is set on fire, or a block that can be lit is lit: fire, a campfire, a candle, TNT...

Returning `true` from a handler prevents the ignition.

Handled by [`Event::on_block_ignite`](crate::prelude::Event::on_block_ignite) and [`Server::on_block_ignite`](crate::server::Server::on_block_ignite).
*/
pub struct BlockIgniteEvent {
    /// Name of the player who lit it, or an empty string when no player did.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block that is lit.
    pub block: u32,
    /// What lit it, such as `flint_and_steel`, `dispenser` or `lightning`.
    pub cause: String,
}
