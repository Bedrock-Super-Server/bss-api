use serde::Serialize;

/// The kind of tool or weapon a custom item is, which sets how it can be enchanted and repaired, and which blocks it mines quickly.
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Material {
    /// A sword.
    Sword,
    /// A pickaxe.
    Pickaxe,
    /// An axe.
    Axe,
    /// A shovel.
    Shovel,
    /// A hoe.
    Hoe,
}
