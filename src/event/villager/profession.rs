/**
A villager takes a profession by claiming a workstation.

`event.cancel()` keeps the villager unemployed.

Handled by [`Event::on_villager_profession`](crate::prelude::Event::on_villager_profession) and [`Server::on_villager_profession`](crate::server::Server::on_villager_profession).
*/
pub struct VillagerProfessionEvent {
    /// Runtime id of the entity.
    pub entity: u64,
    /// The profession, such as `farmer` or `librarian`.
    pub profession: String,
    /// X coordinate of the workstation.
    pub x: i32,
    /// Y coordinate of the workstation.
    pub y: i32,
    /// Z coordinate of the workstation.
    pub z: i32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
