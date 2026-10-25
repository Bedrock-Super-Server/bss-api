/**
The sound a custom block makes when it is placed, broken, walked on, hit or landed on.

A sound type is one the game already has, such as [`Stone`](BlockSound::Stone) or [`Wood`](BlockSound::Wood): the client
plays it for every one of those events, so the server has nothing to send. The names are the ones of the
`minecraft:sound` component of a block, and need a game of version 1.26.50 or later.

Two values are not a sound type:

- [`Default`](BlockSound::Default) leaves the choice to the client, which plays the sound it uses for a block that
  does not say.
- [`Custom`](BlockSound::Custom) is for a plugin that plays its own sounds. The block carries no sound type, and when a
  player places or breaks it the server sends no sound (the breaking particles are the same event, so they are not
  sent either). The client still plays the sounds it makes by itself, such as footsteps, and a block broken by
  anything else than a player, an explosion for instance, keeps the sound the server sends for it.

# Example

```
use bss_api::prelude::*;

struct Ruby;

impl CustomBlock for Ruby {
    fn id(&self) -> String {
        "shop:ruby_block".into()
    }

    fn display(&self) -> String {
        "Block of Ruby".into()
    }

    fn texture(&self) -> String {
        "ruby_block".into()
    }

    fn sound(&self) -> BlockSound {
        BlockSound::AmethystBlock
    }
}
```
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum BlockSound {
    /// The client plays the sound it uses for a block that does not say.
    Default,
    /// The plugin plays its own sounds: the server sends none for this block.
    Custom,
    /// The sound type `amethyst_block`.
    AmethystBlock,
    /// The sound type `amethyst_cluster`.
    AmethystCluster,
    /// The sound type `ancient_debris`.
    AncientDebris,
    /// The sound type `anvil`.
    Anvil,
    /// The sound type `azalea`.
    Azalea,
    /// The sound type `azalea_leaves`.
    AzaleaLeaves,
    /// The sound type `bamboo`.
    Bamboo,
    /// The sound type `bamboo_sapling`.
    BambooSapling,
    /// The sound type `bamboo_wood`.
    BambooWood,
    /// The sound type `bamboo_wood_hanging_sign`.
    BambooWoodHangingSign,
    /// The sound type `basalt`.
    Basalt,
    /// The sound type `big_dripleaf`.
    BigDripleaf,
    /// The sound type `bone_block`.
    BoneBlock,
    /// The sound type `cactus_flower`.
    CactusFlower,
    /// The sound type `calcite`.
    Calcite,
    /// The sound type `candle`.
    Candle,
    /// The sound type `cave_vines`.
    CaveVines,
    /// The sound type `chain`.
    Chain,
    /// The sound type `cherry_leaves`.
    CherryLeaves,
    /// The sound type `cherry_wood`.
    CherryWood,
    /// The sound type `cherry_wood_hanging_sign`.
    CherryWoodHangingSign,
    /// The sound type `chiseled_bookshelf`.
    ChiseledBookshelf,
    /// The sound type `cinnabar`.
    Cinnabar,
    /// The sound type `cloth`.
    Cloth,
    /// The sound type `comparator`.
    Comparator,
    /// The sound type `copper`.
    Copper,
    /// The sound type `copper_bulb`.
    CopperBulb,
    /// The sound type `copper_chest`.
    CopperChest,
    /// The sound type `copper_chest_oxidized`.
    CopperChestOxidized,
    /// The sound type `copper_chest_weathered`.
    CopperChestWeathered,
    /// The sound type `copper_golem_statue`.
    CopperGolemStatue,
    /// The sound type `copper_grate`.
    CopperGrate,
    /// The sound type `coral`.
    Coral,
    /// The sound type `creaking_heart`.
    CreakingHeart,
    /// The sound type `deadbush`.
    Deadbush,
    /// The sound type `decorated_pot`.
    DecoratedPot,
    /// The sound type `deepslate`.
    Deepslate,
    /// The sound type `deepslate_bricks`.
    DeepslateBricks,
    /// The sound type `dirt_with_roots`.
    DirtWithRoots,
    /// The sound type `dried_ghast`.
    DriedGhast,
    /// The sound type `dripstone_block`.
    DripstoneBlock,
    /// The sound type `eyeblossom`.
    Eyeblossom,
    /// The sound type `firefly_bush`.
    FireflyBush,
    /// The sound type `frog_spawn`.
    FrogSpawn,
    /// The sound type `froglight`.
    Froglight,
    /// The sound type `fungus`.
    Fungus,
    /// The sound type `glass`.
    Glass,
    /// The sound type `glow_lichen`.
    GlowLichen,
    /// The sound type `grass`.
    Grass,
    /// The sound type `gravel`.
    Gravel,
    /// The sound type `hanging_roots`.
    HangingRoots,
    /// The sound type `hanging_sign`.
    HangingSign,
    /// The sound type `heavy_core`.
    HeavyCore,
    /// The sound type `honey_block`.
    HoneyBlock,
    /// The sound type `iron`.
    Iron,
    /// The sound type `itemframe`.
    Itemframe,
    /// The sound type `ladder`.
    Ladder,
    /// The sound type `lantern`.
    Lantern,
    /// The sound type `large_amethyst_bud`.
    LargeAmethystBud,
    /// The sound type `leaf_litter`.
    LeafLitter,
    /// The sound type `lever`.
    Lever,
    /// The sound type `lodestone`.
    Lodestone,
    /// The sound type `mangrove_roots`.
    MangroveRoots,
    /// The sound type `medium_amethyst_bud`.
    MediumAmethystBud,
    /// The sound type `metal`.
    Metal,
    /// The sound type `mob_spawner`.
    MobSpawner,
    /// The sound type `moss_block`.
    MossBlock,
    /// The sound type `moss_carpet`.
    MossCarpet,
    /// The sound type `mud`.
    Mud,
    /// The sound type `mud_bricks`.
    MudBricks,
    /// The sound type `muddy_mangrove_roots`.
    MuddyMangroveRoots,
    /// The sound type `nether_brick`.
    NetherBrick,
    /// The sound type `nether_gold_ore`.
    NetherGoldOre,
    /// The sound type `nether_sprouts`.
    NetherSprouts,
    /// The sound type `nether_wart`.
    NetherWart,
    /// The sound type `nether_wood`.
    NetherWood,
    /// The sound type `nether_wood_hanging_sign`.
    NetherWoodHangingSign,
    /// The sound type `netherite`.
    Netherite,
    /// The sound type `netherrack`.
    Netherrack,
    /// The sound type `nylium`.
    Nylium,
    /// The sound type `packed_mud`.
    PackedMud,
    /// The sound type `pale_hanging_moss`.
    PaleHangingMoss,
    /// The sound type `pink_petals`.
    PinkPetals,
    /// The sound type `pointed_dripstone`.
    PointedDripstone,
    /// The sound type `polished_tuff`.
    PolishedTuff,
    /// The sound type `poplar_leaves`.
    PoplarLeaves,
    /// The sound type `potent_sulfur`.
    PotentSulfur,
    /// The sound type `powder_snow`.
    PowderSnow,
    /// The sound type `red_shrub`.
    RedShrub,
    /// The sound type `resin`.
    Resin,
    /// The sound type `resin_brick`.
    ResinBrick,
    /// The sound type `roots`.
    Roots,
    /// The sound type `sand`.
    Sand,
    /// The sound type `scaffolding`.
    Scaffolding,
    /// The sound type `sculk`.
    Sculk,
    /// The sound type `sculk_catalyst`.
    SculkCatalyst,
    /// The sound type `sculk_sensor`.
    SculkSensor,
    /// The sound type `sculk_shrieker`.
    SculkShrieker,
    /// The sound type `sculk_vein`.
    SculkVein,
    /// The sound type `shelf`.
    Shelf,
    /// The sound type `shelf_mushroom`.
    ShelfMushroom,
    /// The sound type `shroomlight`.
    Shroomlight,
    /// The sound type `slime`.
    Slime,
    /// The sound type `small_amethyst_bud`.
    SmallAmethystBud,
    /// The sound type `snow`.
    Snow,
    /// The sound type `soul_sand`.
    SoulSand,
    /// The sound type `soul_soil`.
    SoulSoil,
    /// The sound type `sponge`.
    Sponge,
    /// The sound type `spore_blossom`.
    SporeBlossom,
    /// The sound type `stem`.
    Stem,
    /// The sound type `stone`.
    Stone,
    /// The sound type `straw_bed`.
    StrawBed,
    /// The sound type `sulfur`.
    Sulfur,
    /// The sound type `sulfur_spike`.
    SulfurSpike,
    /// The sound type `suspicious_gravel`.
    SuspiciousGravel,
    /// The sound type `suspicious_sand`.
    SuspiciousSand,
    /// The sound type `sweet_berry_bush`.
    SweetBerryBush,
    /// The sound type `terracotta`.
    Terracotta,
    /// The sound type `trial_spawner`.
    TrialSpawner,
    /// The sound type `tuff`.
    Tuff,
    /// The sound type `tuff_bricks`.
    TuffBricks,
    /// The sound type `turtle_egg`.
    TurtleEgg,
    /// The sound type `vault`.
    Vault,
    /// The sound type `vines`.
    Vines,
    /// The sound type `web`.
    Web,
    /// The sound type `weeping_vines`.
    WeepingVines,
    /// The sound type `wet_sponge`.
    WetSponge,
    /// The sound type `wood`.
    Wood,
}

impl BlockSound {
    /// Every sound type of the game, in alphabetical order. [`Default`](BlockSound::Default) and [`Custom`](BlockSound::Custom) are not in it.
    pub const TYPES: &'static [BlockSound] = &[
        BlockSound::AmethystBlock,
        BlockSound::AmethystCluster,
        BlockSound::AncientDebris,
        BlockSound::Anvil,
        BlockSound::Azalea,
        BlockSound::AzaleaLeaves,
        BlockSound::Bamboo,
        BlockSound::BambooSapling,
        BlockSound::BambooWood,
        BlockSound::BambooWoodHangingSign,
        BlockSound::Basalt,
        BlockSound::BigDripleaf,
        BlockSound::BoneBlock,
        BlockSound::CactusFlower,
        BlockSound::Calcite,
        BlockSound::Candle,
        BlockSound::CaveVines,
        BlockSound::Chain,
        BlockSound::CherryLeaves,
        BlockSound::CherryWood,
        BlockSound::CherryWoodHangingSign,
        BlockSound::ChiseledBookshelf,
        BlockSound::Cinnabar,
        BlockSound::Cloth,
        BlockSound::Comparator,
        BlockSound::Copper,
        BlockSound::CopperBulb,
        BlockSound::CopperChest,
        BlockSound::CopperChestOxidized,
        BlockSound::CopperChestWeathered,
        BlockSound::CopperGolemStatue,
        BlockSound::CopperGrate,
        BlockSound::Coral,
        BlockSound::CreakingHeart,
        BlockSound::Deadbush,
        BlockSound::DecoratedPot,
        BlockSound::Deepslate,
        BlockSound::DeepslateBricks,
        BlockSound::DirtWithRoots,
        BlockSound::DriedGhast,
        BlockSound::DripstoneBlock,
        BlockSound::Eyeblossom,
        BlockSound::FireflyBush,
        BlockSound::FrogSpawn,
        BlockSound::Froglight,
        BlockSound::Fungus,
        BlockSound::Glass,
        BlockSound::GlowLichen,
        BlockSound::Grass,
        BlockSound::Gravel,
        BlockSound::HangingRoots,
        BlockSound::HangingSign,
        BlockSound::HeavyCore,
        BlockSound::HoneyBlock,
        BlockSound::Iron,
        BlockSound::Itemframe,
        BlockSound::Ladder,
        BlockSound::Lantern,
        BlockSound::LargeAmethystBud,
        BlockSound::LeafLitter,
        BlockSound::Lever,
        BlockSound::Lodestone,
        BlockSound::MangroveRoots,
        BlockSound::MediumAmethystBud,
        BlockSound::Metal,
        BlockSound::MobSpawner,
        BlockSound::MossBlock,
        BlockSound::MossCarpet,
        BlockSound::Mud,
        BlockSound::MudBricks,
        BlockSound::MuddyMangroveRoots,
        BlockSound::NetherBrick,
        BlockSound::NetherGoldOre,
        BlockSound::NetherSprouts,
        BlockSound::NetherWart,
        BlockSound::NetherWood,
        BlockSound::NetherWoodHangingSign,
        BlockSound::Netherite,
        BlockSound::Netherrack,
        BlockSound::Nylium,
        BlockSound::PackedMud,
        BlockSound::PaleHangingMoss,
        BlockSound::PinkPetals,
        BlockSound::PointedDripstone,
        BlockSound::PolishedTuff,
        BlockSound::PoplarLeaves,
        BlockSound::PotentSulfur,
        BlockSound::PowderSnow,
        BlockSound::RedShrub,
        BlockSound::Resin,
        BlockSound::ResinBrick,
        BlockSound::Roots,
        BlockSound::Sand,
        BlockSound::Scaffolding,
        BlockSound::Sculk,
        BlockSound::SculkCatalyst,
        BlockSound::SculkSensor,
        BlockSound::SculkShrieker,
        BlockSound::SculkVein,
        BlockSound::Shelf,
        BlockSound::ShelfMushroom,
        BlockSound::Shroomlight,
        BlockSound::Slime,
        BlockSound::SmallAmethystBud,
        BlockSound::Snow,
        BlockSound::SoulSand,
        BlockSound::SoulSoil,
        BlockSound::Sponge,
        BlockSound::SporeBlossom,
        BlockSound::Stem,
        BlockSound::Stone,
        BlockSound::StrawBed,
        BlockSound::Sulfur,
        BlockSound::SulfurSpike,
        BlockSound::SuspiciousGravel,
        BlockSound::SuspiciousSand,
        BlockSound::SweetBerryBush,
        BlockSound::Terracotta,
        BlockSound::TrialSpawner,
        BlockSound::Tuff,
        BlockSound::TuffBricks,
        BlockSound::TurtleEgg,
        BlockSound::Vault,
        BlockSound::Vines,
        BlockSound::Web,
        BlockSound::WeepingVines,
        BlockSound::WetSponge,
        BlockSound::Wood,
    ];

    /// Returns the name of the sound type in the `minecraft:sound` component, or `None` for [`Default`](BlockSound::Default) and [`Custom`](BlockSound::Custom).
    pub const fn name(self) -> Option<&'static str> {
        match self {
            BlockSound::Default | BlockSound::Custom => None,
            BlockSound::AmethystBlock => Some("amethyst_block"),
            BlockSound::AmethystCluster => Some("amethyst_cluster"),
            BlockSound::AncientDebris => Some("ancient_debris"),
            BlockSound::Anvil => Some("anvil"),
            BlockSound::Azalea => Some("azalea"),
            BlockSound::AzaleaLeaves => Some("azalea_leaves"),
            BlockSound::Bamboo => Some("bamboo"),
            BlockSound::BambooSapling => Some("bamboo_sapling"),
            BlockSound::BambooWood => Some("bamboo_wood"),
            BlockSound::BambooWoodHangingSign => Some("bamboo_wood_hanging_sign"),
            BlockSound::Basalt => Some("basalt"),
            BlockSound::BigDripleaf => Some("big_dripleaf"),
            BlockSound::BoneBlock => Some("bone_block"),
            BlockSound::CactusFlower => Some("cactus_flower"),
            BlockSound::Calcite => Some("calcite"),
            BlockSound::Candle => Some("candle"),
            BlockSound::CaveVines => Some("cave_vines"),
            BlockSound::Chain => Some("chain"),
            BlockSound::CherryLeaves => Some("cherry_leaves"),
            BlockSound::CherryWood => Some("cherry_wood"),
            BlockSound::CherryWoodHangingSign => Some("cherry_wood_hanging_sign"),
            BlockSound::ChiseledBookshelf => Some("chiseled_bookshelf"),
            BlockSound::Cinnabar => Some("cinnabar"),
            BlockSound::Cloth => Some("cloth"),
            BlockSound::Comparator => Some("comparator"),
            BlockSound::Copper => Some("copper"),
            BlockSound::CopperBulb => Some("copper_bulb"),
            BlockSound::CopperChest => Some("copper_chest"),
            BlockSound::CopperChestOxidized => Some("copper_chest_oxidized"),
            BlockSound::CopperChestWeathered => Some("copper_chest_weathered"),
            BlockSound::CopperGolemStatue => Some("copper_golem_statue"),
            BlockSound::CopperGrate => Some("copper_grate"),
            BlockSound::Coral => Some("coral"),
            BlockSound::CreakingHeart => Some("creaking_heart"),
            BlockSound::Deadbush => Some("deadbush"),
            BlockSound::DecoratedPot => Some("decorated_pot"),
            BlockSound::Deepslate => Some("deepslate"),
            BlockSound::DeepslateBricks => Some("deepslate_bricks"),
            BlockSound::DirtWithRoots => Some("dirt_with_roots"),
            BlockSound::DriedGhast => Some("dried_ghast"),
            BlockSound::DripstoneBlock => Some("dripstone_block"),
            BlockSound::Eyeblossom => Some("eyeblossom"),
            BlockSound::FireflyBush => Some("firefly_bush"),
            BlockSound::FrogSpawn => Some("frog_spawn"),
            BlockSound::Froglight => Some("froglight"),
            BlockSound::Fungus => Some("fungus"),
            BlockSound::Glass => Some("glass"),
            BlockSound::GlowLichen => Some("glow_lichen"),
            BlockSound::Grass => Some("grass"),
            BlockSound::Gravel => Some("gravel"),
            BlockSound::HangingRoots => Some("hanging_roots"),
            BlockSound::HangingSign => Some("hanging_sign"),
            BlockSound::HeavyCore => Some("heavy_core"),
            BlockSound::HoneyBlock => Some("honey_block"),
            BlockSound::Iron => Some("iron"),
            BlockSound::Itemframe => Some("itemframe"),
            BlockSound::Ladder => Some("ladder"),
            BlockSound::Lantern => Some("lantern"),
            BlockSound::LargeAmethystBud => Some("large_amethyst_bud"),
            BlockSound::LeafLitter => Some("leaf_litter"),
            BlockSound::Lever => Some("lever"),
            BlockSound::Lodestone => Some("lodestone"),
            BlockSound::MangroveRoots => Some("mangrove_roots"),
            BlockSound::MediumAmethystBud => Some("medium_amethyst_bud"),
            BlockSound::Metal => Some("metal"),
            BlockSound::MobSpawner => Some("mob_spawner"),
            BlockSound::MossBlock => Some("moss_block"),
            BlockSound::MossCarpet => Some("moss_carpet"),
            BlockSound::Mud => Some("mud"),
            BlockSound::MudBricks => Some("mud_bricks"),
            BlockSound::MuddyMangroveRoots => Some("muddy_mangrove_roots"),
            BlockSound::NetherBrick => Some("nether_brick"),
            BlockSound::NetherGoldOre => Some("nether_gold_ore"),
            BlockSound::NetherSprouts => Some("nether_sprouts"),
            BlockSound::NetherWart => Some("nether_wart"),
            BlockSound::NetherWood => Some("nether_wood"),
            BlockSound::NetherWoodHangingSign => Some("nether_wood_hanging_sign"),
            BlockSound::Netherite => Some("netherite"),
            BlockSound::Netherrack => Some("netherrack"),
            BlockSound::Nylium => Some("nylium"),
            BlockSound::PackedMud => Some("packed_mud"),
            BlockSound::PaleHangingMoss => Some("pale_hanging_moss"),
            BlockSound::PinkPetals => Some("pink_petals"),
            BlockSound::PointedDripstone => Some("pointed_dripstone"),
            BlockSound::PolishedTuff => Some("polished_tuff"),
            BlockSound::PoplarLeaves => Some("poplar_leaves"),
            BlockSound::PotentSulfur => Some("potent_sulfur"),
            BlockSound::PowderSnow => Some("powder_snow"),
            BlockSound::RedShrub => Some("red_shrub"),
            BlockSound::Resin => Some("resin"),
            BlockSound::ResinBrick => Some("resin_brick"),
            BlockSound::Roots => Some("roots"),
            BlockSound::Sand => Some("sand"),
            BlockSound::Scaffolding => Some("scaffolding"),
            BlockSound::Sculk => Some("sculk"),
            BlockSound::SculkCatalyst => Some("sculk_catalyst"),
            BlockSound::SculkSensor => Some("sculk_sensor"),
            BlockSound::SculkShrieker => Some("sculk_shrieker"),
            BlockSound::SculkVein => Some("sculk_vein"),
            BlockSound::Shelf => Some("shelf"),
            BlockSound::ShelfMushroom => Some("shelf_mushroom"),
            BlockSound::Shroomlight => Some("shroomlight"),
            BlockSound::Slime => Some("slime"),
            BlockSound::SmallAmethystBud => Some("small_amethyst_bud"),
            BlockSound::Snow => Some("snow"),
            BlockSound::SoulSand => Some("soul_sand"),
            BlockSound::SoulSoil => Some("soul_soil"),
            BlockSound::Sponge => Some("sponge"),
            BlockSound::SporeBlossom => Some("spore_blossom"),
            BlockSound::Stem => Some("stem"),
            BlockSound::Stone => Some("stone"),
            BlockSound::StrawBed => Some("straw_bed"),
            BlockSound::Sulfur => Some("sulfur"),
            BlockSound::SulfurSpike => Some("sulfur_spike"),
            BlockSound::SuspiciousGravel => Some("suspicious_gravel"),
            BlockSound::SuspiciousSand => Some("suspicious_sand"),
            BlockSound::SweetBerryBush => Some("sweet_berry_bush"),
            BlockSound::Terracotta => Some("terracotta"),
            BlockSound::TrialSpawner => Some("trial_spawner"),
            BlockSound::Tuff => Some("tuff"),
            BlockSound::TuffBricks => Some("tuff_bricks"),
            BlockSound::TurtleEgg => Some("turtle_egg"),
            BlockSound::Vault => Some("vault"),
            BlockSound::Vines => Some("vines"),
            BlockSound::Web => Some("web"),
            BlockSound::WeepingVines => Some("weeping_vines"),
            BlockSound::WetSponge => Some("wet_sponge"),
            BlockSound::Wood => Some("wood"),
        }
    }

    /// Returns the sound type called `name`, such as `amethyst_block`, or `None` when the game has none by that name.
    pub fn from_name(name: &str) -> Option<BlockSound> {
        Some(match name {
            "amethyst_block" => BlockSound::AmethystBlock,
            "amethyst_cluster" => BlockSound::AmethystCluster,
            "ancient_debris" => BlockSound::AncientDebris,
            "anvil" => BlockSound::Anvil,
            "azalea" => BlockSound::Azalea,
            "azalea_leaves" => BlockSound::AzaleaLeaves,
            "bamboo" => BlockSound::Bamboo,
            "bamboo_sapling" => BlockSound::BambooSapling,
            "bamboo_wood" => BlockSound::BambooWood,
            "bamboo_wood_hanging_sign" => BlockSound::BambooWoodHangingSign,
            "basalt" => BlockSound::Basalt,
            "big_dripleaf" => BlockSound::BigDripleaf,
            "bone_block" => BlockSound::BoneBlock,
            "cactus_flower" => BlockSound::CactusFlower,
            "calcite" => BlockSound::Calcite,
            "candle" => BlockSound::Candle,
            "cave_vines" => BlockSound::CaveVines,
            "chain" => BlockSound::Chain,
            "cherry_leaves" => BlockSound::CherryLeaves,
            "cherry_wood" => BlockSound::CherryWood,
            "cherry_wood_hanging_sign" => BlockSound::CherryWoodHangingSign,
            "chiseled_bookshelf" => BlockSound::ChiseledBookshelf,
            "cinnabar" => BlockSound::Cinnabar,
            "cloth" => BlockSound::Cloth,
            "comparator" => BlockSound::Comparator,
            "copper" => BlockSound::Copper,
            "copper_bulb" => BlockSound::CopperBulb,
            "copper_chest" => BlockSound::CopperChest,
            "copper_chest_oxidized" => BlockSound::CopperChestOxidized,
            "copper_chest_weathered" => BlockSound::CopperChestWeathered,
            "copper_golem_statue" => BlockSound::CopperGolemStatue,
            "copper_grate" => BlockSound::CopperGrate,
            "coral" => BlockSound::Coral,
            "creaking_heart" => BlockSound::CreakingHeart,
            "deadbush" => BlockSound::Deadbush,
            "decorated_pot" => BlockSound::DecoratedPot,
            "deepslate" => BlockSound::Deepslate,
            "deepslate_bricks" => BlockSound::DeepslateBricks,
            "dirt_with_roots" => BlockSound::DirtWithRoots,
            "dried_ghast" => BlockSound::DriedGhast,
            "dripstone_block" => BlockSound::DripstoneBlock,
            "eyeblossom" => BlockSound::Eyeblossom,
            "firefly_bush" => BlockSound::FireflyBush,
            "frog_spawn" => BlockSound::FrogSpawn,
            "froglight" => BlockSound::Froglight,
            "fungus" => BlockSound::Fungus,
            "glass" => BlockSound::Glass,
            "glow_lichen" => BlockSound::GlowLichen,
            "grass" => BlockSound::Grass,
            "gravel" => BlockSound::Gravel,
            "hanging_roots" => BlockSound::HangingRoots,
            "hanging_sign" => BlockSound::HangingSign,
            "heavy_core" => BlockSound::HeavyCore,
            "honey_block" => BlockSound::HoneyBlock,
            "iron" => BlockSound::Iron,
            "itemframe" => BlockSound::Itemframe,
            "ladder" => BlockSound::Ladder,
            "lantern" => BlockSound::Lantern,
            "large_amethyst_bud" => BlockSound::LargeAmethystBud,
            "leaf_litter" => BlockSound::LeafLitter,
            "lever" => BlockSound::Lever,
            "lodestone" => BlockSound::Lodestone,
            "mangrove_roots" => BlockSound::MangroveRoots,
            "medium_amethyst_bud" => BlockSound::MediumAmethystBud,
            "metal" => BlockSound::Metal,
            "mob_spawner" => BlockSound::MobSpawner,
            "moss_block" => BlockSound::MossBlock,
            "moss_carpet" => BlockSound::MossCarpet,
            "mud" => BlockSound::Mud,
            "mud_bricks" => BlockSound::MudBricks,
            "muddy_mangrove_roots" => BlockSound::MuddyMangroveRoots,
            "nether_brick" => BlockSound::NetherBrick,
            "nether_gold_ore" => BlockSound::NetherGoldOre,
            "nether_sprouts" => BlockSound::NetherSprouts,
            "nether_wart" => BlockSound::NetherWart,
            "nether_wood" => BlockSound::NetherWood,
            "nether_wood_hanging_sign" => BlockSound::NetherWoodHangingSign,
            "netherite" => BlockSound::Netherite,
            "netherrack" => BlockSound::Netherrack,
            "nylium" => BlockSound::Nylium,
            "packed_mud" => BlockSound::PackedMud,
            "pale_hanging_moss" => BlockSound::PaleHangingMoss,
            "pink_petals" => BlockSound::PinkPetals,
            "pointed_dripstone" => BlockSound::PointedDripstone,
            "polished_tuff" => BlockSound::PolishedTuff,
            "poplar_leaves" => BlockSound::PoplarLeaves,
            "potent_sulfur" => BlockSound::PotentSulfur,
            "powder_snow" => BlockSound::PowderSnow,
            "red_shrub" => BlockSound::RedShrub,
            "resin" => BlockSound::Resin,
            "resin_brick" => BlockSound::ResinBrick,
            "roots" => BlockSound::Roots,
            "sand" => BlockSound::Sand,
            "scaffolding" => BlockSound::Scaffolding,
            "sculk" => BlockSound::Sculk,
            "sculk_catalyst" => BlockSound::SculkCatalyst,
            "sculk_sensor" => BlockSound::SculkSensor,
            "sculk_shrieker" => BlockSound::SculkShrieker,
            "sculk_vein" => BlockSound::SculkVein,
            "shelf" => BlockSound::Shelf,
            "shelf_mushroom" => BlockSound::ShelfMushroom,
            "shroomlight" => BlockSound::Shroomlight,
            "slime" => BlockSound::Slime,
            "small_amethyst_bud" => BlockSound::SmallAmethystBud,
            "snow" => BlockSound::Snow,
            "soul_sand" => BlockSound::SoulSand,
            "soul_soil" => BlockSound::SoulSoil,
            "sponge" => BlockSound::Sponge,
            "spore_blossom" => BlockSound::SporeBlossom,
            "stem" => BlockSound::Stem,
            "stone" => BlockSound::Stone,
            "straw_bed" => BlockSound::StrawBed,
            "sulfur" => BlockSound::Sulfur,
            "sulfur_spike" => BlockSound::SulfurSpike,
            "suspicious_gravel" => BlockSound::SuspiciousGravel,
            "suspicious_sand" => BlockSound::SuspiciousSand,
            "sweet_berry_bush" => BlockSound::SweetBerryBush,
            "terracotta" => BlockSound::Terracotta,
            "trial_spawner" => BlockSound::TrialSpawner,
            "tuff" => BlockSound::Tuff,
            "tuff_bricks" => BlockSound::TuffBricks,
            "turtle_egg" => BlockSound::TurtleEgg,
            "vault" => BlockSound::Vault,
            "vines" => BlockSound::Vines,
            "web" => BlockSound::Web,
            "weeping_vines" => BlockSound::WeepingVines,
            "wet_sponge" => BlockSound::WetSponge,
            "wood" => BlockSound::Wood,
            _ => return None,
        })
    }
}
