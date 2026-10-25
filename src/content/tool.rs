use serde::Serialize;

/// A family of tools, used to say which tool mines a block quickly.
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolType {
    /// Pickaxes: stone and metal.
    Pickaxe,
    /// Axes: wood and plants.
    Axe,
    /// Shovels: dirt, sand and gravel.
    Shovel,
    /// Hoes: crops and leaves.
    Hoe,
    /// Swords: cobwebs and bamboo.
    Sword,
    /// Shears: wool and leaves.
    Shears,
}
