/**
A player fills or empties a bucket.

Handled by [`Event::on_bucket_use`](crate::prelude::Event::on_bucket_use).
*/
pub struct BucketUseEvent {
    /// Name of the player.
    pub name: String,
    /// Numeric id of the bucket in the player's hand.
    pub item_id: i16,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// `true` when the bucket is filled, `false` when it is emptied.
    pub filling: bool,
}

impl BucketUseEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
