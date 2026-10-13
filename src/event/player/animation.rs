/**
A player plays an animation, such as swinging their arm.

Returning `true` from a handler hides the animation from other players.

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
