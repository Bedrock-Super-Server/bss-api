/**
Water or lava is about to flow into a block.

`event.cancel()` stops the liquid from flowing into this block.

Handled by [`Event::on_liquid_flow`](crate::prelude::Event::on_liquid_flow) and [`Server::on_liquid_flow`](crate::server::Server::on_liquid_flow).
*/
pub struct LiquidFlowEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the liquid block that would be placed.
    pub liquid: u32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
