use super::{ArmorSlot, ItemDef, Material, ToolType};
use crate::server::Server;

/**
An item added by a plugin, registered with [`Server::register_custom_item`].

Only [`id`](Self::id), [`display`](Self::display) and [`texture`](Self::texture) are required; the other methods describe tools, weapons and armor. See the [`content`](crate::content) module for an example.
*/
pub trait CustomItem: Send + Sync + 'static {
    /// Namespaced id of the item, such as `shop:ruby`.
    fn id(&self) -> String;

    /// Name shown in game.
    fn display(&self) -> String;

    /// Name of the item texture in the resource pack the server sends.
    fn texture(&self) -> String;

    /// Largest stack, from `1` to `64`.
    fn max_stack(&self) -> u8 {
        64
    }

    /// Durability of a tool, weapon or armor piece; `0` for an item that does not wear out.
    fn max_durability(&self) -> i16 {
        0
    }

    /// Kind of tool or weapon the item is, if any.
    fn material(&self) -> Option<Material> {
        None
    }

    /// Families of blocks the item mines quickly, at [`mining_speed`](Self::mining_speed).
    fn tools(&self) -> Vec<ToolType> {
        Vec::new()
    }

    /// Melee damage, in half-hearts.
    fn attack_damage(&self) -> f32 {
        1.0
    }

    /// How fast the item mines the blocks of its [`tools`](Self::tools); a bare hand is `1.0`.
    fn mining_speed(&self) -> f32 {
        1.0
    }

    /// Armor points, for an item with an [`armor_slot`](Self::armor_slot).
    fn protection(&self) -> u8 {
        0
    }

    /// Where the item is worn, for armor.
    fn armor_slot(&self) -> Option<ArmorSlot> {
        None
    }

    /// Lines of text shown under the name.
    fn lore(&self) -> Vec<String> {
        Vec::new()
    }

    /// Blocks the item mines at a speed of its own, as `(block id, speed)`.
    fn fast_blocks(&self) -> Vec<(String, i32)> {
        Vec::new()
    }

    /// Whether the item never loses durability.
    fn indestructible(&self) -> bool {
        false
    }

    /// Whether the unique id of each stack is shown as a line of lore, to tell apart copies of a valuable item.
    fn show_uid(&self) -> bool {
        false
    }
}

impl Server {
    /// Registers a [`CustomItem`]. Call it from [`Plugin::register`](crate::sdk::Plugin::register).
    pub fn register_custom_item<C: CustomItem>(&self, item: &C) {
        let mut tools: Vec<ToolType> = Vec::new();
        for tool in item.tools() {
            if !tools.contains(&tool) {
                tools.push(tool);
            }
        }

        self.push_item(&ItemDef {
            id: item.id(),
            display: item.display(),
            texture: item.texture(),
            max_stack: item.max_stack(),
            max_durability: item.max_durability(),
            material: item.material(),
            tools,
            attack_damage: item.attack_damage(),
            mining_speed: item.mining_speed(),
            protection: item.protection(),
            armor_slot: item.armor_slot(),
            lore: item.lore(),
            fast_blocks: item.fast_blocks(),
            indestructible: item.indestructible(),
            show_uid: item.show_uid(),
        });
    }
}
