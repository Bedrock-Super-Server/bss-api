use serde::Serialize;

use super::category::Category;
use super::vanilla_group::VanillaGroup;
use crate::ffi::call::HostFn;
use crate::server::Server;

/// A group of items in a tab of the creative inventory, shown as one expandable entry.
#[derive(Clone, Serialize)]
pub struct GroupDef {
    /// Tab the group appears in.
    pub category: Category,
    /// Name of the group.
    pub name: String,
    /// Item shown on the collapsed group.
    pub icon: String,
    /// Items in the group, in order.
    pub members: Vec<String>,
}

impl GroupDef {
    /// Creates an empty group named `name` in `category`.
    pub fn new(category: Category, name: impl Into<String>) -> Self {
        Self {
            category,
            name: name.into(),
            icon: String::new(),
            members: Vec::new(),
        }
    }

    /// Sets the item shown on the collapsed group.
    pub fn icon(mut self, id: impl Into<String>) -> Self {
        self.icon = id.into();
        self
    }

    /// Appends the item `id` to the group.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, id: impl Into<String>) -> Self {
        self.members.push(id.into());
        self
    }
}

/**
A group of the creative inventory that a plugin defines as a type, and that items and blocks name as theirs.

Implement it on a struct, register it with [`Server::register_custom_group`], and have each item or block return
[`GroupRef::custom`] of that struct from its `group` method: the group holds them in the order they were registered. To
put items in a group of the game instead, see [`VanillaGroup`].

```
use bss_api::prelude::*;

struct Rubies;

impl CustomGroup for Rubies {
    fn name(&self) -> String {
        "Rubies".into()
    }

    fn category(&self) -> Category {
        Category::Items
    }

    fn icon(&self) -> String {
        "shop:ruby".into()
    }
}

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

    fn group(&self) -> Option<GroupRef> {
        Some(GroupRef::custom::<Rubies>())
    }
}

fn register(server: &Server) {
    server.register_custom_group(&Rubies);
    server.register_custom_item(&Ruby);
}
```
*/
pub trait CustomGroup: Send + Sync + 'static {
    /// Name shown on the group.
    fn name(&self) -> String;

    /// Tab of the creative inventory the group appears in.
    fn category(&self) -> Category;

    /// Id of the item shown on the collapsed group; empty shows its first member.
    fn icon(&self) -> String {
        String::new()
    }
}

/// The group an item or a block is in: one of the plugin, see [`CustomGroup`], or one of the game, see [`VanillaGroup`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupRef {
    /// A group of the plugin, by the name of the type that defines it.
    Custom(String),
    /// A group of the game, by its name.
    Vanilla(String),
}

impl GroupRef {
    /// The group that `G` defines. `G` must be registered with [`Server::register_custom_group`].
    pub fn custom<G: CustomGroup>() -> Self {
        GroupRef::Custom(std::any::type_name::<G>().to_string())
    }

    /// A group of the game.
    pub fn vanilla(group: VanillaGroup) -> Self {
        GroupRef::Vanilla(group.name().to_string())
    }
}

#[derive(Serialize)]
struct CustomGroupDef {
    key: String,
    category: Category,
    name: String,
    icon: String,
    members: Vec<String>,
}

impl Server {
    /// Registers a [`CustomGroup`]. Call it from [`Plugin::register`](crate::sdk::Plugin::register), before or after the items and blocks that name it.
    pub fn register_custom_group<G: CustomGroup>(&self, group: &G) {
        let def = CustomGroupDef {
            key: std::any::type_name::<G>().to_string(),
            category: group.category(),
            name: group.name(),
            icon: group.icon(),
            members: Vec::new(),
        };
        if let Some(f) = self.host().content_register_group {
            let json = serde_json::to_string(&def).unwrap_or_default();
            f.call((crate::ffi::abi::FfiStr::new(&json),));
        }
    }
}
