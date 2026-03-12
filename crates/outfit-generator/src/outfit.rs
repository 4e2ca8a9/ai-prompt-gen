use std::collections::HashSet;
use std::fmt;

use rand::seq::SliceRandom;
use rand::Rng;

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

    /// `true` when no items were selected (the person is nude).
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl fmt::Display for Outfit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.items.is_empty() {
            return write!(f, "(nothing)");
        }
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "- {item}")?;
        }
        Ok(())
    }
}

/// The order in which categories are considered. Earlier categories claim
/// slots first so inner layers beat outer layers.
const CATEGORY_ORDER: &[Category] = &[
    Category::Underwear,
    Category::Socks,
    Category::Top,
    Category::Bottom,
    Category::Dress,
    Category::Shoes,
    Category::Outerwear,
    Category::Accessory,
    Category::Jewelry,
];

/// Generate a random outfit from the wardrobe.
///
/// For each category (in layering order), every compatible item has a random
/// chance of being included. Each item's slots are checked against what is
/// already occupied — if there is a conflict it is skipped. Any slot may end
/// up unfilled, so the result can range from fully dressed to completely nude.
pub fn generate_outfit(wardrobe: &Wardrobe) -> Outfit {
    let mut rng = rand::thread_rng();
    let mut occupied: HashSet<Slot> = HashSet::new();
    let mut chosen: Vec<Item> = Vec::new();

    for &category in CATEGORY_ORDER {
        let mut candidates: Vec<&Item> = wardrobe
            .items_in(category)
            .into_iter()
            .filter(|item| is_compatible(item, &occupied))
            .collect();

        candidates.shuffle(&mut rng);

        for item in candidates {
            if !is_compatible(item, &occupied) {
                // A previously accepted item in this same loop iteration may
                // have claimed a slot this candidate needs.
                continue;
            }

            if rng.gen_bool(0.5) {
                claim_slots(item, &mut occupied);
                chosen.push(item.clone());
            }
        }
    }

    Outfit { items: chosen }
}

/// Returns `true` if `candidate` can be added without conflicting with any
/// already-occupied slot.
fn is_compatible(candidate: &Item, occupied: &HashSet<Slot>) -> bool {
    candidate.slots.iter().all(|s| !occupied.contains(s))
}

/// Mark all of `item`'s slots as occupied.
fn claim_slots(item: &Item, occupied: &mut HashSet<Slot>) {
    occupied.extend(item.slots.iter().copied());
}
