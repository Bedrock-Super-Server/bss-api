/**
A portal is about to be created: a Nether portal is lit or generated for travel, or an End portal is completed.

`event.cancel()` prevents the portal from being created.

Handled by [`Event::on_portal_create`](crate::prelude::Event::on_portal_create) and [`Server::on_portal_create`](crate::server::Server::on_portal_create).
*/
pub struct PortalCreateEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Width of the portal, in blocks.
    pub width: u32,
    /// Height of the portal, in blocks.
    pub height: u32,
    /// Axis of the portal: `x`, `z`, or `y` for an End portal.
    pub axis: String,
    /// Why, such as `fire`, `travel` or `ender_eye`.
    pub cause: String,
    /// Name of the player who created it, or an empty string.
    pub player: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl PortalCreateEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
