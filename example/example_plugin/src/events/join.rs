use bss_api::prelude::{Event, JoinEvent, Server};

pub struct Welcome;

impl Event for Welcome {
    fn on_join(&self, _server: &Server, event: &JoinEvent) {
        event.set_message(&format!("§e{} joined the game, welcome!", event.name));
    }
}
