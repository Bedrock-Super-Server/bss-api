use serde::Serialize;

use super::armor_slot::ArmorSlot;
use super::material::Material;
use super::tool::ToolType;

#[derive(Serialize)]
pub(crate) struct ItemDef {
    pub id: String,
    pub display: String,
    pub texture: String,
    pub max_stack: u8,
    pub max_durability: i16,
    pub material: Option<Material>,
    pub tools: Vec<ToolType>,
    pub attack_damage: f32,
    pub mining_speed: f32,
    pub protection: u8,
    pub armor_slot: Option<ArmorSlot>,
    pub lore: Vec<String>,
    pub fast_blocks: Vec<(String, i32)>,
    pub indestructible: bool,
    pub show_uid: bool,
}
