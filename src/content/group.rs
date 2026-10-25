use serde::Serialize;

use super::category::Category;

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
