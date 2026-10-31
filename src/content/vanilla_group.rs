use serde::Serialize;

use super::category::Category;

/**
A group the game itself shows in the creative inventory, such as the planks or the stairs, where a plugin can add its own items and blocks:
[`CustomItem::group`](super::CustomItem::group) and [`CustomBlock::group`](super::CustomBlock::group) return a [`GroupRef::vanilla`](super::GroupRef::vanilla) of one.

The items of the game that are not in a named group of their own, the loose ones of each tab, are not listed here.

```
use bss_api::prelude::*;

struct RubyPlanks;

impl CustomBlock for RubyPlanks {
    fn id(&self) -> String {
        "shop:ruby_planks".into()
    }

    fn display(&self) -> String {
        "Ruby Planks".into()
    }

    fn texture(&self) -> String {
        "planks_oak".into()
    }

    // Shown with the planks of the game.
    fn group(&self) -> Option<GroupRef> {
        Some(GroupRef::vanilla(VanillaGroup::Planks))
    }
}
```
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub enum VanillaGroup {
    /// The `planks` group of the construction tab.
    Planks,
    /// The `walls` group of the construction tab.
    Walls,
    /// The `fence` group of the construction tab.
    Fence,
    /// The `fenceGate` group of the construction tab.
    FenceGate,
    /// The `stairs` group of the construction tab.
    Stairs,
    /// The `door` group of the construction tab.
    Door,
    /// The `trapdoor` group of the construction tab.
    Trapdoor,
    /// The `bars` group of the construction tab.
    Bars,
    /// The `glass` group of the construction tab.
    Glass,
    /// The `glassPane` group of the construction tab.
    GlassPane,
    /// The `slab` group of the construction tab.
    Slab,
    /// The `stoneBrick` group of the construction tab.
    StoneBrick,
    /// The `sandstone` group of the construction tab.
    Sandstone,
    /// The `copper` group of the construction tab.
    Copper,
    /// The `wool` group of the construction tab.
    Wool,
    /// The `woolCarpet` group of the construction tab.
    WoolCarpet,
    /// The `concretePowder` group of the construction tab.
    ConcretePowder,
    /// The `concrete` group of the construction tab.
    Concrete,
    /// The `stainedClay` group of the construction tab.
    StainedClay,
    /// The `glazedTerracotta` group of the construction tab.
    GlazedTerracotta,
    /// The `ore` group of the nature tab.
    Ore,
    /// The `stone` group of the nature tab.
    Stone,
    /// The `log` group of the nature tab.
    Log,
    /// The `wood` group of the nature tab.
    Wood,
    /// The `leaves` group of the nature tab.
    Leaves,
    /// The `sapling` group of the nature tab.
    Sapling,
    /// The `seed` group of the nature tab.
    Seed,
    /// The `crop` group of the nature tab.
    Crop,
    /// The `grass` group of the nature tab.
    Grass,
    /// The `coral_decorations` group of the nature tab.
    CoralDecorations,
    /// The `flower` group of the nature tab.
    Flower,
    /// The `dye` group of the nature tab.
    Dye,
    /// The `rawFood` group of the nature tab.
    RawFood,
    /// The `mushroom` group of the nature tab.
    Mushroom,
    /// The `monsterStoneEgg` group of the nature tab.
    MonsterStoneEgg,
    /// The `mobEgg` group of the nature tab.
    MobEgg,
    /// The `coral` group of the nature tab.
    Coral,
    /// The `sculk` group of the nature tab.
    Sculk,
    /// The `helmet` group of the equipment tab.
    Helmet,
    /// The `chestplate` group of the equipment tab.
    Chestplate,
    /// The `leggings` group of the equipment tab.
    Leggings,
    /// The `boots` group of the equipment tab.
    Boots,
    /// The `sword` group of the equipment tab.
    Sword,
    /// The `spear` group of the equipment tab.
    Spear,
    /// The `axe` group of the equipment tab.
    Axe,
    /// The `pickaxe` group of the equipment tab.
    Pickaxe,
    /// The `shovel` group of the equipment tab.
    Shovel,
    /// The `hoe` group of the equipment tab.
    Hoe,
    /// The `arrow` group of the equipment tab.
    Arrow,
    /// The `cookedFood` group of the equipment tab.
    CookedFood,
    /// The `miscFood` group of the equipment tab.
    MiscFood,
    /// The `goatHorn` group of the equipment tab.
    GoatHorn,
    /// The `harnesses` group of the equipment tab.
    Harnesses,
    /// The `bundles` group of the equipment tab.
    Bundles,
    /// The `horseArmor` group of the equipment tab.
    HorseArmor,
    /// The `nautilus_armor` group of the equipment tab.
    NautilusArmor,
    /// The `potion` group of the equipment tab.
    Potion,
    /// The `splashPotion` group of the equipment tab.
    SplashPotion,
    /// The `lingeringPotion` group of the equipment tab.
    LingeringPotion,
    /// The `ominousBottle` group of the equipment tab.
    OminousBottle,
    /// The `bed` group of the items tab.
    Bed,
    /// The `lanterns` group of the items tab.
    Lanterns,
    /// The `candles` group of the items tab.
    Candles,
    /// The `anvil` group of the items tab.
    Anvil,
    /// The `shelf` group of the items tab.
    Shelf,
    /// The `chest` group of the items tab.
    Chest,
    /// The `shulkerBox` group of the items tab.
    ShulkerBox,
    /// The `copper_golem_statue` group of the items tab.
    CopperGolemStatue,
    /// The `record` group of the items tab.
    Record,
    /// The `sign` group of the items tab.
    Sign,
    /// The `hanging_sign` group of the items tab.
    HangingSign,
    /// The `skull` group of the items tab.
    Skull,
    /// The `lightning_rod` group of the items tab.
    LightningRod,
    /// The `enchantedBook` group of the items tab.
    EnchantedBook,
    /// The `boat` group of the items tab.
    Boat,
    /// The `chestboat` group of the items tab.
    Chestboat,
    /// The `rail` group of the items tab.
    Rail,
    /// The `minecart` group of the items tab.
    Minecart,
    /// The `buttons` group of the items tab.
    Buttons,
    /// The `pressurePlate` group of the items tab.
    PressurePlate,
    /// The `banner` group of the items tab.
    Banner,
    /// The `banner_pattern` group of the items tab.
    BannerPattern,
    /// The `potterySherds` group of the items tab.
    PotterySherds,
    /// The `smithing_templates` group of the items tab.
    SmithingTemplates,
    /// The `firework` group of the items tab.
    Firework,
    /// The `fireworkStars` group of the items tab.
    FireworkStars,
    /// The `chains` group of the items tab.
    Chains,
}

impl VanillaGroup {
    /// Every group.
    pub const ALL: &[VanillaGroup] = &[
        VanillaGroup::Planks,
        VanillaGroup::Walls,
        VanillaGroup::Fence,
        VanillaGroup::FenceGate,
        VanillaGroup::Stairs,
        VanillaGroup::Door,
        VanillaGroup::Trapdoor,
        VanillaGroup::Bars,
        VanillaGroup::Glass,
        VanillaGroup::GlassPane,
        VanillaGroup::Slab,
        VanillaGroup::StoneBrick,
        VanillaGroup::Sandstone,
        VanillaGroup::Copper,
        VanillaGroup::Wool,
        VanillaGroup::WoolCarpet,
        VanillaGroup::ConcretePowder,
        VanillaGroup::Concrete,
        VanillaGroup::StainedClay,
        VanillaGroup::GlazedTerracotta,
        VanillaGroup::Ore,
        VanillaGroup::Stone,
        VanillaGroup::Log,
        VanillaGroup::Wood,
        VanillaGroup::Leaves,
        VanillaGroup::Sapling,
        VanillaGroup::Seed,
        VanillaGroup::Crop,
        VanillaGroup::Grass,
        VanillaGroup::CoralDecorations,
        VanillaGroup::Flower,
        VanillaGroup::Dye,
        VanillaGroup::RawFood,
        VanillaGroup::Mushroom,
        VanillaGroup::MonsterStoneEgg,
        VanillaGroup::MobEgg,
        VanillaGroup::Coral,
        VanillaGroup::Sculk,
        VanillaGroup::Helmet,
        VanillaGroup::Chestplate,
        VanillaGroup::Leggings,
        VanillaGroup::Boots,
        VanillaGroup::Sword,
        VanillaGroup::Spear,
        VanillaGroup::Axe,
        VanillaGroup::Pickaxe,
        VanillaGroup::Shovel,
        VanillaGroup::Hoe,
        VanillaGroup::Arrow,
        VanillaGroup::CookedFood,
        VanillaGroup::MiscFood,
        VanillaGroup::GoatHorn,
        VanillaGroup::Harnesses,
        VanillaGroup::Bundles,
        VanillaGroup::HorseArmor,
        VanillaGroup::NautilusArmor,
        VanillaGroup::Potion,
        VanillaGroup::SplashPotion,
        VanillaGroup::LingeringPotion,
        VanillaGroup::OminousBottle,
        VanillaGroup::Bed,
        VanillaGroup::Lanterns,
        VanillaGroup::Candles,
        VanillaGroup::Anvil,
        VanillaGroup::Shelf,
        VanillaGroup::Chest,
        VanillaGroup::ShulkerBox,
        VanillaGroup::CopperGolemStatue,
        VanillaGroup::Record,
        VanillaGroup::Sign,
        VanillaGroup::HangingSign,
        VanillaGroup::Skull,
        VanillaGroup::LightningRod,
        VanillaGroup::EnchantedBook,
        VanillaGroup::Boat,
        VanillaGroup::Chestboat,
        VanillaGroup::Rail,
        VanillaGroup::Minecart,
        VanillaGroup::Buttons,
        VanillaGroup::PressurePlate,
        VanillaGroup::Banner,
        VanillaGroup::BannerPattern,
        VanillaGroup::PotterySherds,
        VanillaGroup::SmithingTemplates,
        VanillaGroup::Firework,
        VanillaGroup::FireworkStars,
        VanillaGroup::Chains,
    ];

    /// The name the game gives the group, such as `itemGroup.name.planks`.
    pub const fn name(self) -> &'static str {
        match self {
            VanillaGroup::Planks => "itemGroup.name.planks",
            VanillaGroup::Walls => "itemGroup.name.walls",
            VanillaGroup::Fence => "itemGroup.name.fence",
            VanillaGroup::FenceGate => "itemGroup.name.fenceGate",
            VanillaGroup::Stairs => "itemGroup.name.stairs",
            VanillaGroup::Door => "itemGroup.name.door",
            VanillaGroup::Trapdoor => "itemGroup.name.trapdoor",
            VanillaGroup::Bars => "itemGroup.name.bars",
            VanillaGroup::Glass => "itemGroup.name.glass",
            VanillaGroup::GlassPane => "itemGroup.name.glassPane",
            VanillaGroup::Slab => "itemGroup.name.slab",
            VanillaGroup::StoneBrick => "itemGroup.name.stoneBrick",
            VanillaGroup::Sandstone => "itemGroup.name.sandstone",
            VanillaGroup::Copper => "itemGroup.name.copper",
            VanillaGroup::Wool => "itemGroup.name.wool",
            VanillaGroup::WoolCarpet => "itemGroup.name.woolCarpet",
            VanillaGroup::ConcretePowder => "itemGroup.name.concretePowder",
            VanillaGroup::Concrete => "itemGroup.name.concrete",
            VanillaGroup::StainedClay => "itemGroup.name.stainedClay",
            VanillaGroup::GlazedTerracotta => "itemGroup.name.glazedTerracotta",
            VanillaGroup::Ore => "itemGroup.name.ore",
            VanillaGroup::Stone => "itemGroup.name.stone",
            VanillaGroup::Log => "itemGroup.name.log",
            VanillaGroup::Wood => "itemGroup.name.wood",
            VanillaGroup::Leaves => "itemGroup.name.leaves",
            VanillaGroup::Sapling => "itemGroup.name.sapling",
            VanillaGroup::Seed => "itemGroup.name.seed",
            VanillaGroup::Crop => "itemGroup.name.crop",
            VanillaGroup::Grass => "itemGroup.name.grass",
            VanillaGroup::CoralDecorations => "itemGroup.name.coral_decorations",
            VanillaGroup::Flower => "itemGroup.name.flower",
            VanillaGroup::Dye => "itemGroup.name.dye",
            VanillaGroup::RawFood => "itemGroup.name.rawFood",
            VanillaGroup::Mushroom => "itemGroup.name.mushroom",
            VanillaGroup::MonsterStoneEgg => "itemGroup.name.monsterStoneEgg",
            VanillaGroup::MobEgg => "itemGroup.name.mobEgg",
            VanillaGroup::Coral => "itemGroup.name.coral",
            VanillaGroup::Sculk => "itemGroup.name.sculk",
            VanillaGroup::Helmet => "itemGroup.name.helmet",
            VanillaGroup::Chestplate => "itemGroup.name.chestplate",
            VanillaGroup::Leggings => "itemGroup.name.leggings",
            VanillaGroup::Boots => "itemGroup.name.boots",
            VanillaGroup::Sword => "itemGroup.name.sword",
            VanillaGroup::Spear => "itemGroup.name.spear",
            VanillaGroup::Axe => "itemGroup.name.axe",
            VanillaGroup::Pickaxe => "itemGroup.name.pickaxe",
            VanillaGroup::Shovel => "itemGroup.name.shovel",
            VanillaGroup::Hoe => "itemGroup.name.hoe",
            VanillaGroup::Arrow => "itemGroup.name.arrow",
            VanillaGroup::CookedFood => "itemGroup.name.cookedFood",
            VanillaGroup::MiscFood => "itemGroup.name.miscFood",
            VanillaGroup::GoatHorn => "itemGroup.name.goatHorn",
            VanillaGroup::Harnesses => "itemGroup.name.harnesses",
            VanillaGroup::Bundles => "itemGroup.name.bundles",
            VanillaGroup::HorseArmor => "itemGroup.name.horseArmor",
            VanillaGroup::NautilusArmor => "itemGroup.name.nautilus_armor",
            VanillaGroup::Potion => "itemGroup.name.potion",
            VanillaGroup::SplashPotion => "itemGroup.name.splashPotion",
            VanillaGroup::LingeringPotion => "itemGroup.name.lingeringPotion",
            VanillaGroup::OminousBottle => "itemGroup.name.ominousBottle",
            VanillaGroup::Bed => "itemGroup.name.bed",
            VanillaGroup::Lanterns => "itemGroup.name.lanterns",
            VanillaGroup::Candles => "itemGroup.name.candles",
            VanillaGroup::Anvil => "itemGroup.name.anvil",
            VanillaGroup::Shelf => "itemGroup.name.shelf",
            VanillaGroup::Chest => "itemGroup.name.chest",
            VanillaGroup::ShulkerBox => "itemGroup.name.shulkerBox",
            VanillaGroup::CopperGolemStatue => "itemGroup.name.copper_golem_statue",
            VanillaGroup::Record => "itemGroup.name.record",
            VanillaGroup::Sign => "itemGroup.name.sign",
            VanillaGroup::HangingSign => "itemGroup.name.hanging_sign",
            VanillaGroup::Skull => "itemGroup.name.skull",
            VanillaGroup::LightningRod => "itemGroup.name.lightning_rod",
            VanillaGroup::EnchantedBook => "itemGroup.name.enchantedBook",
            VanillaGroup::Boat => "itemGroup.name.boat",
            VanillaGroup::Chestboat => "itemGroup.name.chestboat",
            VanillaGroup::Rail => "itemGroup.name.rail",
            VanillaGroup::Minecart => "itemGroup.name.minecart",
            VanillaGroup::Buttons => "itemGroup.name.buttons",
            VanillaGroup::PressurePlate => "itemGroup.name.pressurePlate",
            VanillaGroup::Banner => "itemGroup.name.banner",
            VanillaGroup::BannerPattern => "itemGroup.name.banner_pattern",
            VanillaGroup::PotterySherds => "itemGroup.name.potterySherds",
            VanillaGroup::SmithingTemplates => "itemGroup.name.smithing_templates",
            VanillaGroup::Firework => "itemGroup.name.firework",
            VanillaGroup::FireworkStars => "itemGroup.name.fireworkStars",
            VanillaGroup::Chains => "itemGroup.name.chains",
        }
    }

    /// The tab of the creative inventory the group is in.
    pub const fn category(self) -> Category {
        match self {
            VanillaGroup::Planks
            | VanillaGroup::Walls
            | VanillaGroup::Fence
            | VanillaGroup::FenceGate
            | VanillaGroup::Stairs
            | VanillaGroup::Door
            | VanillaGroup::Trapdoor
            | VanillaGroup::Bars
            | VanillaGroup::Glass
            | VanillaGroup::GlassPane
            | VanillaGroup::Slab
            | VanillaGroup::StoneBrick
            | VanillaGroup::Sandstone
            | VanillaGroup::Copper
            | VanillaGroup::Wool
            | VanillaGroup::WoolCarpet
            | VanillaGroup::ConcretePowder
            | VanillaGroup::Concrete
            | VanillaGroup::StainedClay
            | VanillaGroup::GlazedTerracotta => Category::Construction,
            VanillaGroup::Ore
            | VanillaGroup::Stone
            | VanillaGroup::Log
            | VanillaGroup::Wood
            | VanillaGroup::Leaves
            | VanillaGroup::Sapling
            | VanillaGroup::Seed
            | VanillaGroup::Crop
            | VanillaGroup::Grass
            | VanillaGroup::CoralDecorations
            | VanillaGroup::Flower
            | VanillaGroup::Dye
            | VanillaGroup::RawFood
            | VanillaGroup::Mushroom
            | VanillaGroup::MonsterStoneEgg
            | VanillaGroup::MobEgg
            | VanillaGroup::Coral
            | VanillaGroup::Sculk => Category::Nature,
            VanillaGroup::Helmet
            | VanillaGroup::Chestplate
            | VanillaGroup::Leggings
            | VanillaGroup::Boots
            | VanillaGroup::Sword
            | VanillaGroup::Spear
            | VanillaGroup::Axe
            | VanillaGroup::Pickaxe
            | VanillaGroup::Shovel
            | VanillaGroup::Hoe
            | VanillaGroup::Arrow
            | VanillaGroup::CookedFood
            | VanillaGroup::MiscFood
            | VanillaGroup::GoatHorn
            | VanillaGroup::Harnesses
            | VanillaGroup::Bundles
            | VanillaGroup::HorseArmor
            | VanillaGroup::NautilusArmor
            | VanillaGroup::Potion
            | VanillaGroup::SplashPotion
            | VanillaGroup::LingeringPotion
            | VanillaGroup::OminousBottle => Category::Equipment,
            VanillaGroup::Bed
            | VanillaGroup::Lanterns
            | VanillaGroup::Candles
            | VanillaGroup::Anvil
            | VanillaGroup::Shelf
            | VanillaGroup::Chest
            | VanillaGroup::ShulkerBox
            | VanillaGroup::CopperGolemStatue
            | VanillaGroup::Record
            | VanillaGroup::Sign
            | VanillaGroup::HangingSign
            | VanillaGroup::Skull
            | VanillaGroup::LightningRod
            | VanillaGroup::EnchantedBook
            | VanillaGroup::Boat
            | VanillaGroup::Chestboat
            | VanillaGroup::Rail
            | VanillaGroup::Minecart
            | VanillaGroup::Buttons
            | VanillaGroup::PressurePlate
            | VanillaGroup::Banner
            | VanillaGroup::BannerPattern
            | VanillaGroup::PotterySherds
            | VanillaGroup::SmithingTemplates
            | VanillaGroup::Firework
            | VanillaGroup::FireworkStars
            | VanillaGroup::Chains => Category::Items,
        }
    }
}
