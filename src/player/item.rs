/// An enchantment, in the order of the Bedrock enchantment ids.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enchant {
    /// Protection.
    Protection,
    /// Fire protection.
    FireProtection,
    /// Feather falling.
    FeatherFalling,
    /// Blast protection.
    BlastProtection,
    /// Projectile protection.
    ProjectileProtection,
    /// Thorns.
    Thorns,
    /// Respiration.
    Respiration,
    /// Depth strider.
    DepthStrider,
    /// Aqua affinity.
    AquaAffinity,
    /// Sharpness.
    Sharpness,
    /// Smite.
    Smite,
    /// Bane of arthropods.
    BaneOfArthropods,
    /// Knockback.
    Knockback,
    /// Fire aspect.
    FireAspect,
    /// Looting.
    Looting,
    /// Efficiency.
    Efficiency,
    /// Silk touch.
    SilkTouch,
    /// Unbreaking.
    Unbreaking,
    /// Fortune.
    Fortune,
    /// Power.
    Power,
    /// Punch.
    Punch,
    /// Flame.
    Flame,
    /// Infinity.
    Infinity,
    /// Luck of the sea.
    LuckOfTheSea,
    /// Lure.
    Lure,
    /// Frost walker.
    FrostWalker,
    /// Mending.
    Mending,
    /// Curse of binding.
    CurseOfBinding,
    /// Curse of vanishing.
    CurseOfVanishing,
    /// Impaling.
    Impaling,
    /// Riptide.
    Riptide,
    /// Loyalty.
    Loyalty,
    /// Channeling.
    Channeling,
    /// Multishot.
    Multishot,
    /// Piercing.
    Piercing,
    /// Quick charge.
    QuickCharge,
    /// Soul speed.
    SoulSpeed,
    /// Swift sneak.
    SwiftSneak,
}

impl Enchant {
    /// Returns the Bedrock id of the enchantment.
    pub fn id(self) -> i16 {
        self as i16
    }
}

/**
An item stack with an optional custom name, lore and enchantments, built step by step.

```
use bss_api::player::{Enchant, Item};

let pickaxe = Item::new(321)
    .name("§6Golden Touch")
    .lore(vec!["Mines faster.".into()])
    .enchant(Enchant::Efficiency, 5)
    .unbreakable();
assert_eq!(pickaxe.count, 1);
assert_eq!(pickaxe.enchants, vec![(Enchant::Efficiency.id(), 5)]);
```
*/
#[derive(Clone, Default)]
pub struct Item {
    /// Numeric id of the item.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0`.
    pub block_runtime_id: u32,
    /// Size of the stack.
    pub count: u8,
    /// Damage of a tool, weapon or armor piece, or data value of other items.
    pub damage: u16,
    /// Custom name; empty keeps the default name.
    pub name: String,
    /// Lines of text shown under the name.
    pub lore: Vec<String>,
    /// Enchantments, as `(id, level)`.
    pub enchants: Vec<(i16, i16)>,
    /// Whether the item never loses durability.
    pub unbreakable: bool,
}

impl Item {
    /// One item of `item_id`, with no name, lore or enchantment.
    pub fn new(item_id: i16) -> Self {
        Self {
            item_id,
            block_runtime_id: 0,
            count: 1,
            damage: 0,
            name: String::new(),
            lore: Vec::new(),
            enchants: Vec::new(),
            unbreakable: false,
        }
    }

    /// One item that places the block `block_runtime_id`.
    pub fn block(item_id: i16, block_runtime_id: u32) -> Self {
        Self {
            block_runtime_id,
            ..Self::new(item_id)
        }
    }

    /// Sets the size of the stack.
    pub fn count(mut self, count: u8) -> Self {
        self.count = count;
        self
    }

    /// Sets the damage or data value.
    pub fn damage(mut self, damage: u16) -> Self {
        self.damage = damage;
        self
    }

    /// Sets the custom name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Sets the lines of lore.
    pub fn lore(mut self, lore: Vec<String>) -> Self {
        self.lore = lore;
        self
    }

    /// Adds an enchantment.
    pub fn enchant(mut self, enchant: Enchant, level: i16) -> Self {
        self.enchants.push((enchant.id(), level));
        self
    }

    /// Adds an enchantment by its Bedrock id, for one [`Enchant`] does not list.
    pub fn enchant_raw(mut self, id: i16, level: i16) -> Self {
        self.enchants.push((id, level));
        self
    }

    /// Makes the item unbreakable.
    pub fn unbreakable(mut self) -> Self {
        self.unbreakable = true;
        self
    }

    pub(crate) fn lore_json(&self) -> String {
        serde_json::to_string(&self.lore).unwrap_or_default()
    }

    pub(crate) fn ench_json(&self) -> String {
        serde_json::to_string(&self.enchants).unwrap_or_default()
    }
}

/// The name, lore and enchantments of a stack in an inventory, as [`Inventory::get_meta`](crate::player::Inventory::get_meta) returns them.
#[derive(Clone, Default, serde::Deserialize)]
pub struct ItemMeta {
    /// Custom name, or an empty string.
    #[serde(default)]
    pub name: String,
    /// Lines of lore.
    #[serde(default)]
    pub lore: Vec<String>,
    /// Enchantments, as `(id, level)`.
    #[serde(default)]
    pub enchants: Vec<(i16, i16)>,
    /// Whether the item never loses durability.
    #[serde(default)]
    pub unbreakable: bool,
}

impl ItemMeta {
    pub(crate) fn from_json(json: &str) -> Self {
        if json.is_empty() {
            return Self::default();
        }
        serde_json::from_str(json).unwrap_or_default()
    }
}
