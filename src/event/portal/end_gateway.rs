/**
An End gateway is about to be generated after a dragon fight.

`event.cancel()` prevents the gateway.

Handled by [`Event::on_end_gateway`](crate::prelude::Event::on_end_gateway) and [`Server::on_end_gateway`](crate::server::Server::on_end_gateway).
*/
pub struct EndGatewayEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
