use std::collections::{HashMap, HashSet};
use std::fmt;

use rand::seq::SliceRandom;
use rand::Rng;

use crate::item::{Category, Item, Slot};
use crate::preset::Preset;
use crate::variation::OutfitItem;
use crate::wardrobe::Wardrobe;

/// A generated outfit — a set of items (with variations assigned) that can
/// all be worn together without any slot conflicts.
#[derive(Debug, Clone)]
pub struct Outfit {
    items: Vec<OutfitItem>,
}

impl Outfit {
    /// The items that make up this outfit.
    pub fn items(&self) -> &[OutfitItem] {
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

/// Generate a random outfit from the entire wardrobe.
pub fn generate_outfit(wardrobe: &Wardrobe) -> Outfit {
    build_outfit(wardrobe, None)
}

/// Generate a random outfit using only items listed in a preset.
pub fn generate_outfit_from_preset(wardrobe: &Wardrobe, preset: &Preset) -> Outfit {
    build_outfit(wardrobe, Some(preset))
}

fn build_outfit(wardrobe: &Wardrobe, preset: Option<&Preset>) -> Outfit {
    let mut rng = rand::thread_rng();

    let allowed_slugs: Option<HashSet<&str>> =
        preset.map(|p| p.items.iter().map(|s| s.as_str()).collect());

    // --- Phase 1: pick items ---
    let mut occupied: HashSet<Slot> = HashSet::new();
    let mut chosen: Vec<Item> = Vec::new();

    for &category in CATEGORY_ORDER {
        let mut candidates: Vec<&Item> = wardrobe
            .items_in(category)
            .into_iter()
            .filter(|item| {
                if let Some(ref allowed) = allowed_slugs {
                    if !allowed.contains(item.slug.as_str()) {
                        return false;
                    }
                }
                is_compatible(item, &occupied)
            })
            .collect();

        candidates.shuffle(&mut rng);

        for item in candidates {
            if !is_compatible(item, &occupied) {
                continue;
            }
            if rng.gen_bool(0.5) {
                claim_slots(item, &mut occupied);
                chosen.push(item.clone());
            }
        }
    }

    // --- Phase 2: assign variations ---
    let match_set: Option<&HashMap<String, String>> = wardrobe
        .match_sets()
        .choose(&mut rng)
        .map(|ms| &ms.set);

    let var_cats = wardrobe.variation_categories();

    let items = chosen
        .into_iter()
        .map(|item| {
            let mut assigned = HashMap::new();
            for var_cat_name in &item.variations {
                // Try the match set first.
                if let Some(set) = match_set {
                    if let Some(value) = set.get(var_cat_name) {
                        assigned.insert(var_cat_name.clone(), value.clone());
                        continue;
                    }
                }
                // Fall back to a random option from the category.
                if let Some(cat) = var_cats.get(var_cat_name) {
                    if let Some(value) = cat.options.choose(&mut rng) {
                        assigned.insert(var_cat_name.clone(), value.clone());
                    }
                }
            }
            OutfitItem {
                slug: item.slug,
                name: item.name,
                variations: assigned,
            }
        })
        .collect();

    Outfit { items }
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
