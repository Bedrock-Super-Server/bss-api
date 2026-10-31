/**
A player edits or signs a book and quill.

`event.cancel()` rejects the edit.

Handled by [`Event::on_book_edit`](crate::prelude::Event::on_book_edit) and [`Server::on_book_edit`](crate::server::Server::on_book_edit).
*/
pub struct BookEditEvent {
    /// Name of the player.
    pub player: String,
    /// Inventory slot of the book.
    pub slot: u32,
    /// What changed: `replace_page`, `add_page`, `delete_page`, `swap_pages` or `sign`.
    pub action: String,
    /// Page concerned, counted from `0`, or `-1` when the book is signed.
    pub page: i32,
    /// New text of the page; for `swap_pages`, the index of the other page; for `sign`, the title.
    pub text: String,
}

impl BookEditEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
