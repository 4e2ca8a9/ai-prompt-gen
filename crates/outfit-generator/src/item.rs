use serde::{Deserialize, Serialize};
use std::fmt;

/// A slot on the body that an item can occupy.
/// Two items conflict if they share any slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Slot {
    // Head / face
    Head,
    Hair,
    Ears,
    Eyes,

    // Neck / torso
    Neck,
    TorsoUnder,
    Torso,
    TorsoOuter,

    // Lower body
    Crotch,
    LegsUnder,
    Legs,
    LowerLegs,
    Waist,

    // Hands / wrists
    WristLeft,
    WristRight,
    Fingers,

    // Feet
    Ankles,
    FeetInner,
    FeetOuter,

    // Carried
    Bag,
}

/// The broad category an item belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Underwear,
    Top,
    Bottom,
    Dress,
    Outerwear,
    Accessory,
    Jewelry,
    Socks,
    Shoes,
}

/// A single clothing or accessory item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Unique identifier, e.g. `"dress.wrap_dress"`.
    pub slug: String,
    /// Display name, e.g. "Wrap Dress".
    pub name: String,
    /// Which broad category this item falls into.
    pub category: Category,
    /// Body slots this item occupies. Items that share a slot conflict.
    pub slots: Vec<Slot>,
    /// Variation categories that apply to this item (e.g. `["fabric"]`,
    /// `["metal"]`, `["fabric", "leather"]`). Empty means no variations.
    #[serde(default)]
    pub variations: Vec<String>,
}

impl Item {
    /// Returns `true` if wearing `self` and `other` at the same time would
    /// create a conflict (i.e. they compete for at least one body slot).
    pub fn conflicts_with(&self, other: &Item) -> bool {
        self.slots.iter().any(|s| other.slots.contains(s))
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Helper used when deserializing a TOML file that contains a list of items.
///
/// ```toml
/// [[items]]
/// name = "Stud Earrings"
/// category = "jewelry"
/// slots = ["ears"]
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct ItemFile {
    pub items: Vec<Item>,
}
