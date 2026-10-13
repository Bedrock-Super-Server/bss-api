/**
A villager reaches a new trading level.

Handled by [`Event::on_villager_level_up`](crate::prelude::Event::on_villager_level_up) and [`Server::on_villager_level_up`](crate::server::Server::on_villager_level_up).
*/
pub struct VillagerLevelUpEvent {
    /// Runtime id of the entity.
    pub entity: u64,
    /// New level, from `2` (apprentice) to `5` (master).
    pub level: u32,
    /// Profession of the villager, such as `farmer`.
    pub profession: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
