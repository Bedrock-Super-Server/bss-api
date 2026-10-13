/**
A mob is leashed or unleashed.

Returning `true` from a handler leaves the leash as it was.

Handled by [`Event::on_mob_leash`](crate::prelude::Event::on_mob_leash) and [`Server::on_mob_leash`](crate::server::Server::on_mob_leash).
*/
pub struct MobLeashEvent {
    /// Name of the player, or an empty string when no player is involved.
    pub player: String,
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// `true` when the leash is attached, `false` when it comes off.
    pub leashed: bool,
    /// `true` when the leash is tied to or untied from a fence.
    pub fence: bool,
    /// X coordinate of the fence, when [`fence`](Self::fence) is `true`.
    pub x: i32,
    /// Y coordinate of the fence, when [`fence`](Self::fence) is `true`.
    pub y: i32,
    /// Z coordinate of the fence, when [`fence`](Self::fence) is `true`.
    pub z: i32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
