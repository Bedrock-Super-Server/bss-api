/// A hit to give a player with [`Player::damage`](super::Player::damage).
///
/// Armor reduces the amount, as for a melee hit. Say where the hit comes from to knock the player back
/// away from that point, and who struck to name them in the death message.
///
/// ```
/// use bss_api::prelude::*;
///
/// fn smack(victim: &Player, attacker: &Player) -> ApiResult<bool> {
///     let from = attacker.location();
///     victim.damage(
///         Damage::new(4.0)
///             .from(from.x, from.z)
///             .knockback(0.6, 0.4)
///             .by(attacker.name()),
///     )
/// }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Damage {
    pub(crate) amount: f32,
    pub(crate) from: Option<(f32, f32)>,
    pub(crate) force: f32,
    pub(crate) vertical: f32,
    pub(crate) killer: String,
}

/// How hard a hit that comes from somewhere pushes, until [`Damage::knockback`] says otherwise.
const FORCE: f32 = 0.4;
const VERTICAL: f32 = 0.4;

impl Damage {
    /// A hit of `amount` half-hearts, with no knockback and no one to blame.
    pub fn new(amount: f32) -> Self {
        Self {
            amount,
            from: None,
            force: FORCE,
            vertical: VERTICAL,
            killer: String::new(),
        }
    }

    /// The hit comes from `(x, z)`: the player is knocked back away from there.
    pub fn from(mut self, x: f32, z: f32) -> Self {
        self.from = Some((x, z));
        self
    }

    /// How hard the hit pushes, sideways and up. Only a hit that comes from somewhere pushes.
    pub fn knockback(mut self, force: f32, vertical: f32) -> Self {
        self.force = force;
        self.vertical = vertical;
        self
    }

    /// Who struck, named in the death message when the hit kills.
    pub fn by(mut self, killer: impl Into<String>) -> Self {
        self.killer = killer.into();
        self
    }
}
