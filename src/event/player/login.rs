use crate::player::client_data::ClientData;
use crate::player::jwt::{AuthInfo, ClientDataJwt, Jwt};

/**
A client has logged in, before it joins the world.

Handled by [`Event::on_login`](crate::prelude::Event::on_login).
*/
pub struct LoginEvent {
    /// Name of the player.
    pub name: String,
    /// Xbox user id of the player.
    pub xuid: String,
    /// UUID of the player, in its usual text form.
    pub uuid: String,
    /// IP address and port of the client.
    pub ip: String,
    /// `true` when Xbox Live signed the identity of the player.
    pub authenticated: bool,
    /// Device and skin data the client sent.
    pub client_data: ClientData,
    /// Protocol version the client speaks.
    pub protocol: u32,
    /// Protocol version the server talks to it.
    pub negotiated_protocol: u32,
    /// What the authentication chain says about the player.
    pub auth_info: AuthInfo,
    /// The identity token, decoded.
    pub token: Jwt,
    /// The client data token, decoded.
    pub client_data_token: ClientDataJwt,
}
