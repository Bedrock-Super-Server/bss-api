use bss_api::prelude::{ChatEvent, Event, Server};

const FORBIDDEN: &str = "forbidden";

pub struct ChatFilter;

impl Event for ChatFilter {
    fn on_chat(&self, server: &Server, event: &ChatEvent) -> bool {
        if !event.message.to_ascii_lowercase().contains(FORBIDDEN) {
            return false;
        }
        if let Some(player) = server.player(&event.name) {
            player.message("§cThat word is not allowed here.");
        }
        true
    }
}
