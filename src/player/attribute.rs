/// An attribute of a player, read with [`Player::attribute`](crate::player::Player::attribute) and changed with [`Player::set_attribute`](crate::player::Player::set_attribute).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attribute {
    /// Hunger points, from `0` to `20`.
    Hunger,
    /// Saturation, which drains before hunger does.
    Saturation,
    /// Exhaustion, which drains saturation when it reaches `4`.
    Exhaustion,
    /// Experience level.
    XpLevel,
    /// Progress towards the next level, from `0.0` to `1.0`.
    Xp,
    /// Walking speed; `0.1` by default.
    MovementSpeed,
    /// Swimming speed.
    UnderwaterMovement,
    /// Speed in lava.
    LavaMovement,
    /// Melee damage of a bare hand.
    AttackDamage,
    /// Extra health from absorption, in half-hearts.
    Absorption,
    /// Luck, which improves loot.
    Luck,
    /// Knockback resistance, from `0.0` to `1.0`.
    KnockbackResistance,
}

impl Attribute {
    pub(crate) const fn index(self) -> u32 {
        match self {
            Attribute::Hunger => 0,
            Attribute::Saturation => 1,
            Attribute::Exhaustion => 2,
            Attribute::XpLevel => 3,
            Attribute::Xp => 4,
            Attribute::MovementSpeed => 5,
            Attribute::UnderwaterMovement => 6,
            Attribute::LavaMovement => 7,
            Attribute::AttackDamage => 8,
            Attribute::Absorption => 9,
            Attribute::Luck => 10,
            Attribute::KnockbackResistance => 11,
        }
    }
}
