/**
A player crafts an item.

`event.cancel()` cancels the craft.

Handled by [`Event::on_craft_item`](crate::prelude::Event::on_craft_item) and [`Server::on_craft_item`](crate::server::Server::on_craft_item).
*/
pub struct CraftItemEvent {
    /// Name of the player.
    pub player: String,
    /// Where the item is crafted, such as `crafting_table`, `stonecutter` or `loom`.
    pub station: String,
    /// Id of the recipe used.
    pub recipe: u32,
    /// Numeric id of the crafted item.
    pub item: i16,
    /// Number of items crafted.
    pub count: u32,
}

impl CraftItemEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
