/**
The ender dragon changes its behaviour phase.

Handled by [`Event::on_dragon_phase`](crate::prelude::Event::on_dragon_phase) and [`Server::on_dragon_phase`](crate::server::Server::on_dragon_phase).
*/
pub struct DragonPhaseEvent {
    /// Runtime id of the dragon.
    pub entity: u64,
    /// The new phase, such as `holding_pattern`, `strafe_player`, `landing` or `charging_player`.
    pub phase: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
