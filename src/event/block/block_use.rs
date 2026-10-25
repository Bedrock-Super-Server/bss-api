/**
A player uses a block: opens a door or a container, presses a button, flips a lever, rings a bell...

Returning `true` from a handler cancels the use: the block does not react.

Handled by [`Event::on_block_use`](crate::prelude::Event::on_block_use) and [`Server::on_block_use`](crate::server::Server::on_block_use).
*/
pub struct BlockUseEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block; [`Server::block_name`](crate::server::Server::block_name) gives its name.
    pub block: u32,
    /// Numeric id of the item in the player's hand, or `0` for an empty hand.
    pub item: i16,
    /// What the use does, as named by the server, such as `open` or `toggle`.
    pub action: String,
}
