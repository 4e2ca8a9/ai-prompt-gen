use std::collections::HashSet;
use std::fmt;

use crate::error::Error;
use crate::item::{Category, Item, Slot};
use crate::wardrobe::Wardrobe;

/// A generated outfit — a set of items that can all be worn together
/// without any slot conflicts.
#[derive(Debug, Clone)]
pub struct Outfit {
    items: Vec<Item>,
}

impl Outfit {
    /// The items that make up this outfit.
    pub fn items(&self) -> &[Item] {
        &self.items
    }
}

impl fmt::Display for Outfit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "- {item}")?;
        }
        Ok(())
    }
}

/// Which categories are required vs optional when building an outfit.
struct LayerRules {
    /// Categories where exactly one item must be picked (order matters for
    /// slot-claiming priority: earlier categories claim slots first).
    required: Vec<Category>,
    /// Categories where zero or more non-conflicting items may be added.
    optional: Vec<Category>,
}

impl Default for LayerRules {
    fn default() -> Self {
        Self {
            required: vec![
                // Undergarments first — they claim inner slots.
                Category::Underwear,
                // Core outfit layer.
                Category::Top,
                Category::Bottom,
                // Footwear.
                Category::Socks,
                Category::Shoes,
            ],
            optional: vec![
                Category::Dress,
                Category::Outerwear,
                Category::Accessory,
                Category::Jewelry,
            ],
        }
    }
}

/// Generate a random outfit from the wardrobe.
///
/// The algorithm:
/// 1. For each **required** category (underwear, top, bottom, socks, shoes),
///    pick exactly one item at random whose slots don't conflict with anything
///    already chosen.
/// 2. For each **optional** category (dress, outerwear, accessories, jewelry),
///    try to add zero or more items whose slots don't conflict.
/// 3. A dress occupies both `Torso` and `Legs`, so picking a dress will skip
///    the separate top + bottom requirement.
///
/// Returns an error if a required category cannot be satisfied.
pub fn generate_outfit(_wardrobe: &Wardrobe) -> Result<Outfit, Error> {
    let _rules = LayerRules::default();
    let _occupied: HashSet<Slot> = HashSet::new();
    // TODO: implement selection logic
    //  - iterate required categories, pick a random compatible item
    //  - iterate optional categories, greedily add compatible items
    //  - handle Dress specially (it replaces Top + Bottom)
    todo!()
}

/// Returns `true` if `candidate` can be added without conflicting with any
/// already-occupied slot.
fn _is_compatible(candidate: &Item, occupied: &HashSet<Slot>) -> bool {
    candidate.slots.iter().all(|s| !occupied.contains(s))
}

/// Mark all of `item`'s slots as occupied.
fn _claim_slots(item: &Item, occupied: &mut HashSet<Slot>) {
    occupied.extend(item.slots.iter().copied());
}
