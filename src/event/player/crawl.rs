/**
A player starts or stops crawling.

`event.cancel()` changes nothing.

Handled by [`Event::on_player_crawl`](crate::prelude::Event::on_player_crawl) and [`Server::on_player_crawl`](crate::server::Server::on_player_crawl).
*/
pub struct PlayerCrawlEvent {
    /// Name of the player.
    pub player: String,
    /// Whether the player crawls now.
    pub crawling: bool,
}

impl PlayerCrawlEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
