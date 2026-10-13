/**
A world is saved to disk.

Handled by [`Event::on_world_save`](crate::prelude::Event::on_world_save).
*/
pub struct WorldSaveEvent {
    /// Name of the world.
    pub name: String,
}
