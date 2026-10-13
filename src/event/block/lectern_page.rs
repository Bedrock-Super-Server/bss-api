use std::cell::Cell;

/**
A player turns the page of a book on a lectern.

Returning `true` from a handler keeps the current page.

Handled by [`Event::on_lectern_page`](crate::prelude::Event::on_lectern_page) and [`Server::on_lectern_page`](crate::server::Server::on_lectern_page).
*/
pub struct LecternPageEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    pub(crate) page: Cell<i32>,
}

impl LecternPageEvent {
    /// Returns the page the book opens at, counted from `0`.
    pub fn page(&self) -> i32 {
        self.page.get()
    }

    /// Sets the page the book opens at, counted from `0`.
    pub fn set_page(&self, value: i32) {
        self.page.set(value);
    }
}
