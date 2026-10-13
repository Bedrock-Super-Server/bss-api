/**
A player edits or signs a book and quill.

Returning `true` from a handler rejects the edit.

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
