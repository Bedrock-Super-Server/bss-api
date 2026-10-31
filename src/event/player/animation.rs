/**
A player plays an animation, such as swinging their arm.

`event.cancel()` hides the animation from other players.

Handled by [`Event::on_animation`](crate::prelude::Event::on_animation).
*/
pub struct AnimationEvent {
    /// Name of the player.
    pub name: String,
    /// The animation, as the Bedrock protocol numbers it: `1` is an arm swing.
    pub action: u32,
    /// Extra value sent with the animation.
    pub data: f32,
}

impl AnimationEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
