/*!
Custom items, blocks, recipes and creative inventory groups.

The server sends custom content to players when they join, along with the resource pack that holds its textures. Register content in [`Plugin::register`](crate::sdk::Plugin::register), before any player joins.

# Example

A ruby, a ruby block, the recipe between them, and a creative group (see [`CustomGroup`] to define a group as a type that items and blocks name as theirs, and [`VanillaGroup`] to put them in a group of the game):

```
use bss_api::prelude::*;

struct Ruby;

impl CustomItem for Ruby {
    fn id(&self) -> String {
        "shop:ruby".into()
    }

    fn display(&self) -> String {
        "Ruby".into()
    }

    fn texture(&self) -> String {
        "ruby".into()
    }
}

struct RubyBlock;

impl CustomBlock for RubyBlock {
    fn id(&self) -> String {
        "shop:ruby_block".into()
    }

    fn display(&self) -> String {
        "Block of Ruby".into()
    }

    fn texture(&self) -> String {
        "ruby_block".into()
    }

    fn tool(&self) -> Option<ToolType> {
        Some(ToolType::Pickaxe)
    }
}

fn register(server: &Server) {
    server.register_custom_item(&Ruby);
    server.register_custom_block(&RubyBlock);
    server.register_recipe(&RecipeDef::shaped(
        3,
        3,
        &["shop:ruby"; 9],
        "shop:ruby_block",
        1,
    ));
    server.register_group(&GroupDef::new(Category::Construction, "Rubies").icon("shop:ruby").add("shop:ruby").add("shop:ruby_block"));
}
```

# Sounds

A block makes the sound of a [`BlockSound`]: one of the sound types of the game, [`Default`](BlockSound::Default) to let the
client choose, or [`Custom`](BlockSound::Custom) to play your own.

```
use bss_api::prelude::*;

struct Amethyst;

impl CustomBlock for Amethyst {
    fn id(&self) -> String {
        "shop:amethyst".into()
    }

    fn display(&self) -> String {
        "Amethyst".into()
    }

    fn texture(&self) -> String {
        "amethyst".into()
    }

    fn sound(&self) -> BlockSound {
        BlockSound::AmethystBlock
    }
}
```

# Shapes

A [`BlockShape`] is a form the server builds, places and breaks for you: [`Slab`](BlockShape::Slab),
[`Stairs`](BlockShape::Stairs), [`Trapdoor`](BlockShape::Trapdoor), [`Fence`](BlockShape::Fence),
[`FenceGate`](BlockShape::FenceGate), [`Wall`](BlockShape::Wall), [`Pane`](BlockShape::Pane) and
[`Door`](BlockShape::Door). The server gives the block the states, the boxes and a model of its own, so that any texture of
the game is enough and no resource pack is needed:

```
use bss_api::prelude::*;

struct DirtFence;

impl CustomBlock for DirtFence {
    fn id(&self) -> String {
        "custom:fence".into()
    }

    fn display(&self) -> String {
        "Dirt Fence".into()
    }

    fn texture(&self) -> String {
        "dirt".into()
    }

    fn shape(&self) -> BlockShape {
        BlockShape::Fence
    }
}
```

Fences, panes and walls link to the blocks beside them, gates and doors open with a click, a door is placed from its item as
two blocks, and a slab or a stair is placed on the half and facing the way the game does. Give the block a
[`geometry`](CustomBlock::geometry) of a resource pack of your own to draw it differently: the states and the boxes stay.

# States and permutations

A block lists its own [`BlockState`]s, and the [`BlockPermutation`]s that change its look and its boxes for some of them. It
exists once for each combination of the values of its states. [`bone_visibility`](CustomBlock::bone_visibility) shows a bone
of the geometry only when a [`Condition`] holds, as a fence shows the rails of the sides it links.

```
use bss_api::prelude::*;

struct Toggle;

impl CustomBlock for Toggle {
    fn id(&self) -> String {
        "custom:toggle".into()
    }

    fn display(&self) -> String {
        "Dirt Toggle".into()
    }

    fn texture(&self) -> String {
        "dirt".into()
    }

    fn states(&self) -> Vec<BlockState> {
        vec![BlockState::bool("custom:up"), BlockState::ints("custom:level", 0..=2)]
    }

    fn permutations(&self) -> Vec<BlockPermutation> {
        vec![BlockPermutation::when(Condition::state_is("custom:up", true)).translation([0.0, 0.5, 0.0])]
    }
}
```

A plugin reads and sets the states of a block with [`Block`](crate::server::world::Block), and writes the result back with
[`Server::set_block`](crate::server::Server::set_block):

```no_run
use bss_api::prelude::*;

fn switch(server: &Server, x: i32, y: i32, z: i32) {
    let block = Block::from_runtime_id(server.blocks().at(x, y, z).0);
    let up = block.state_bool("custom:up").unwrap_or(false);
    server.blocks().set(x, y, z, block.with("custom:up", !up).runtime_id());
}
```
*/

mod armor_slot;
mod block_shape;
mod block_sound;
mod block_state;
mod category;
mod custom_block;
mod custom_item;
mod group;
mod material;
mod permutation;
mod recipe;
mod tool;
mod vanilla_group;

pub(crate) mod block;
pub(crate) mod item;

pub use armor_slot::ArmorSlot;
pub use block_shape::BlockShape;
pub use block_sound::BlockSound;
pub use block_state::{BlockState, StateValues};
pub use category::Category;
pub use custom_block::CustomBlock;
pub use custom_item::CustomItem;
pub use group::{CustomGroup, GroupDef, GroupRef};
pub use material::Material;
pub use permutation::{BlockPermutation, Condition, StateValue};
pub use recipe::RecipeDef;
pub use tool::ToolType;
pub use vanilla_group::VanillaGroup;

pub(crate) use block::BlockDef;
pub(crate) use item::ItemDef;

use crate::ffi::abi::FfiStr;
use crate::ffi::call::HostFn;
use crate::server::Server;

impl Server {
    pub(crate) fn push_item(&self, def: &ItemDef) {
        if let Some(f) = self.host().content_register_item {
            let json = serde_json::to_string(def).unwrap_or_default();
            f.call((FfiStr::new(&json),));
        }
    }

    pub(crate) fn push_block(&self, def: &BlockDef) {
        if let Some(f) = self.host().content_register_block {
            let json = serde_json::to_string(def).unwrap_or_default();
            f.call((FfiStr::new(&json),));
        }
    }

    /// Adds a group to the creative inventory. See the [`content`](crate::content) module.
    pub fn register_group(&self, def: &GroupDef) {
        if let Some(f) = self.host().content_register_group {
            let json = serde_json::to_string(def).unwrap_or_default();
            f.call((FfiStr::new(&json),));
        }
    }

    /**
    Adds a recipe of another station than the crafting table: `furnace`, `blast_furnace`,
    `smoker`, `campfire`, `soul_campfire` or `stonecutter`, which take a single input, or
    `crafting_table`.

    ```
    use bss_api::prelude::*;

    fn register(server: &Server) {
        // Smelting a ruby ore gives a ruby, in a furnace and in a blast furnace.
        for station in ["furnace", "blast_furnace"] {
            server.register_recipe_at(station, &RecipeDef::shapeless(&["shop:ruby_ore"], "shop:ruby", 1));
        }
    }
    ```
    */
    pub fn register_recipe_at(&self, station: &str, def: &RecipeDef) -> bool {
        let Some(f) = self.host().recipe_register_at else {
            return false;
        };
        let mut json = serde_json::to_value(def).unwrap_or_default();
        json["station"] = serde_json::Value::String(station.to_string());
        f.call((FfiStr::new(&json.to_string()),));
        true
    }

    /// Adds a crafting table recipe. See the [`content`](crate::content) module.
    pub fn register_recipe(&self, def: &RecipeDef) {
        if let Some(f) = self.host().recipe_register {
            let json = serde_json::to_string(def).unwrap_or_default();
            f.call((FfiStr::new(&json),));
        }
    }
}
