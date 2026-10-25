use serde::Serialize;

/// A tab of the creative inventory.
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// The construction tab.
    Construction,
    /// The nature tab.
    Nature,
    /// The equipment tab.
    Equipment,
    /// The items tab.
    Items,
}
