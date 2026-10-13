use serde::Serialize;

/// The body part an armor item protects.
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ArmorSlot {
    /// Helmets.
    Head,
    /// Chestplates.
    Chest,
    /// Leggings.
    Legs,
    /// Boots.
    Feet,
}
