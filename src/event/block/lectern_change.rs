/**
A player places a book on a lectern or takes it.

Returning `true` from a handler leaves the lectern as it was.

Handled by [`Event::on_lectern_change`](crate::prelude::Event::on_lectern_change) and [`Server::on_lectern_change`](crate::server::Server::on_lectern_change).
*/
pub struct LecternChangeEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Numeric id of the book.
    pub book: i16,
    /// `true` when the book is placed, `false` when it is taken.
    pub placed: bool,
}
