/*!
Custom items, blocks, recipes and creative inventory groups.

The server sends custom content to players when they join, along with the resource pack that holds its textures. Register content in [`Plugin::register`](crate::sdk::Plugin::register), before any player joins.

# Example

A ruby, a ruby block, the recipe between them, and a creative group:

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
*/

mod armor_slot;
mod category;
mod custom_block;
mod custom_item;
mod group;
mod material;
mod recipe;
mod tool;

pub(crate) mod block;
pub(crate) mod item;

pub use armor_slot::ArmorSlot;
pub use category::Category;
pub use custom_block::CustomBlock;
pub use custom_item::CustomItem;
pub use group::GroupDef;
pub use material::Material;
pub use recipe::RecipeDef;
pub use tool::ToolType;

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

    /// Adds a crafting table recipe. See the [`content`](crate::content) module.
    pub fn register_recipe(&self, def: &RecipeDef) {
        if let Some(f) = self.host().recipe_register {
            let json = serde_json::to_string(def).unwrap_or_default();
            f.call((FfiStr::new(&json),));
        }
    }
}
