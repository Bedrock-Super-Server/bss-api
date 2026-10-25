/**
A game packet passes between a player and the server.

Only packets of the directions a plugin subscribed to are reported.

Returning `true` from a handler drops the packet.

Handled by [`Event::on_client_bound`](crate::prelude::Event::on_client_bound).
*/
pub struct PacketEvent {
    /// Runtime id of the player.
    pub entity_id: u64,
    /// Name of the packet, such as `MovePlayer`.
    pub name: String,
    /// Id of the packet in the Bedrock protocol.
    pub packet_id: u32,
    /// Payload of the packet, without its header.
    pub data: Vec<u8>,
    /// Name of the player the packet comes from or goes to.
    pub player: String,
}

impl PacketEvent {
    /**
    The packet as the server decodes it, for the protocol version the player speaks: its fields
    as JSON, by the names [`Player::send_packet`](crate::player::Player::send_packet) takes.
    Returns why it could not be decoded otherwise.

    ```
    use bss_api::prelude::*;

    struct Spy;

    impl Event for Spy {
        fn on_server_bound(&self, _server: &Server, event: &PacketEvent) -> bool {
            if event.name == "Text" {
                if let Ok(fields) = event.fields() {
                    println!("{} wrote {}", event.entity_id, fields["body"]);
                }
            }
            false
        }
    }
    ```
    */
    pub fn fields(&self) -> Result<serde_json::Value, String> {
        let server = crate::registry::server();
        let json = host_call!(
            server.host(),
            packet_decode(
                self.entity_id,
                self.packet_id,
                self.data.as_ptr(),
                self.data.len()
            )
        )
        .map(crate::server::ffi_string)
        .ok_or_else(|| "the server does not decode packets".to_string())?;
        let mut packet: serde_json::Value =
            serde_json::from_str(&json).map_err(|error| error.to_string())?;
        if let Some(error) = packet.get("error").and_then(serde_json::Value::as_str) {
            return Err(error.to_string());
        }
        Ok(packet["fields"].take())
    }
}
