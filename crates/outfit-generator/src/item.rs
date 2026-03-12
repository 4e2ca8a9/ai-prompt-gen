use serde::Deserialize;
use std::fmt;

/// A slot on the body that an item can occupy.
/// Two items conflict if they share any slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
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
    LegsUnder,
    Legs,
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
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
#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    /// Display name, e.g. "White Cotton Bra".
    pub name: String,
    /// Which broad category this item falls into.
    pub category: Category,
    /// Body slots this item occupies. Items that share a slot conflict.
    pub slots: Vec<Slot>,
    /// Freeform tags for filtering / theming (e.g. "casual", "formal", "summer").
    #[serde(default)]
    pub tags: Vec<String>,
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
/// tags = ["casual", "formal"]
/// ```
#[derive(Debug, Deserialize)]
pub struct ItemFile {
    pub items: Vec<Item>,
}
