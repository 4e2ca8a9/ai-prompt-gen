use std::collections::HashSet;
use std::fmt::Write;

use crate::item::{Category, Slot};
use crate::outfit::Outfit;
use crate::wardrobe::Wardrobe;

/// Whether the description is from the front or back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewSide {
    Front,
    Back,
}

/// Generate a natural-English description of an outfit.
///
/// - `name`: the person's name (e.g., "Amy")
/// - `outfit`: the generated outfit
/// - `wardrobe`: used to look up item metadata (category, slots)
/// - `visible`: set of body slots that are visible; items on non-visible slots are omitted,
///   and nakedness is only mentioned for visible areas
/// - `side`: front or back — determines which exposed body parts are mentioned
pub fn describe_outfit(
    name: &str,
    outfit: &Outfit,
    wardrobe: &Wardrobe,
    visible: &HashSet<Slot>,
    side: ViewSide,
) -> String {
    // Resolve each outfit item to its category and slots.
    struct Described {
        formatted: String,
        category: Category,
        slots: Vec<Slot>,
    }

    let mut all_items: Vec<Described> = Vec::new();
    for oi in outfit.items() {
        if let Some(item_def) = wardrobe.item_by_slug(&oi.slug) {
            // Build formatted name: prepend variation values to item name.
            // e.g., "Burgundy Velvet" + "Crop Top" → "Burgundy Velvet Crop Top"
            let mut parts = Vec::new();
            let mut var_values: Vec<_> = oi.variations.values().collect();
            var_values.sort();
            for v in var_values {
                parts.push(v.as_str());
            }
            parts.push(&oi.name);
            let formatted = parts.join(" ");

            all_items.push(Described {
                formatted,
                category: item_def.category,
                slots: item_def.slots.clone(),
            });
        }
    }

    // Track which slots are occupied by non-underwear items (for coverage).
    let outer_slots: HashSet<Slot> = all_items
        .iter()
        .filter(|d| d.category != Category::Underwear)
        .flat_map(|d| d.slots.iter().copied())
        .collect();

    // Track which slots are occupied by underwear (for body-part exposure).
    let underwear_slots: HashSet<Slot> = all_items
        .iter()
        .filter(|d| d.category == Category::Underwear)
        .flat_map(|d| d.slots.iter().copied())
        .collect();

    // Filter: skip items on non-visible slots and covered underwear.
    let items: Vec<Described> = all_items
        .into_iter()
        .filter(|d| {
            // At least one of the item's slots must be visible.
            if !d.slots.iter().any(|s| visible.contains(s)) {
                return false;
            }
            // Hide underwear that's covered by outer layers.
            if d.category == Category::Underwear {
                let covered = d.slots.iter().all(|s| match s {
                    Slot::TorsoUnder => {
                        outer_slots.contains(&Slot::Torso)
                            || outer_slots.contains(&Slot::TorsoOuter)
                    }
                    Slot::Crotch => outer_slots.contains(&Slot::Legs),
                    _ => false,
                });
                if covered {
                    return false;
                }
            }
            true
        })
        .collect();

    // Check visibility of body areas.
    let torso_visible = visible.contains(&Slot::Torso)
        || visible.contains(&Slot::TorsoOuter)
        || visible.contains(&Slot::TorsoUnder);
    let legs_visible = visible.contains(&Slot::Legs)
        || visible.contains(&Slot::Crotch)
        || visible.contains(&Slot::LegsUnder);
    let feet_visible =
        visible.contains(&Slot::FeetOuter) || visible.contains(&Slot::FeetInner);

    // Detect nakedness.
    let has_torso_cover = items.iter().any(|d| {
        matches!(
            d.category,
            Category::Top | Category::Dress | Category::Outerwear
        ) && d
            .slots
            .iter()
            .any(|s| matches!(s, Slot::Torso | Slot::TorsoOuter))
    });
    let has_bottom_cover = items
        .iter()
        .any(|d| matches!(d.category, Category::Bottom | Category::Dress));
    let has_shoes = items.iter().any(|d| d.category == Category::Shoes);

    let topless = torso_visible && !has_torso_cover;
    let bottomless = legs_visible && !has_bottom_cover;
    let barefoot = feet_visible && !has_shoes;
    let nude = topless && bottomless;

    // Determine exposed body parts based on view side and underwear coverage.
    let has_bra = underwear_slots.contains(&Slot::TorsoUnder);
    let has_panties = underwear_slots.contains(&Slot::Crotch);

    let mut exposed: Vec<&str> = Vec::new();
    if topless && torso_visible {
        match side {
            ViewSide::Front => {
                if !has_bra {
                    exposed.push("nipples");
                }
            }
            ViewSide::Back => {
                if !has_bra {
                    exposed.push("back");
                }
            }
        }
    }
    if bottomless && legs_visible {
        match side {
            ViewSide::Front => {
                if !has_panties {
                    exposed.push("genitals");
                }
            }
            ViewSide::Back => {
                if !has_panties {
                    exposed.push("butt");
                }
            }
        }
    }

    // Group items by role.
    let mut clothing: Vec<&str> = Vec::new();
    let mut footwear: Vec<&str> = Vec::new();
    let mut accessories: Vec<&str> = Vec::new();

    for d in &items {
        match d.category {
            Category::Underwear
            | Category::Top
            | Category::Bottom
            | Category::Dress
            | Category::Outerwear => {
                clothing.push(&d.formatted);
            }
            Category::Shoes | Category::Socks => {
                footwear.push(&d.formatted);
            }
            Category::Accessory | Category::Jewelry | Category::Restraints => {
                accessories.push(&d.formatted);
            }
        }
    }

    // Completely nude with nothing at all.
    if nude && clothing.is_empty() && footwear.is_empty() && accessories.is_empty() {
        let mut s = format!("{name} is nude.");
        append_exposed(&mut s, name, &exposed);
        return s;
    }

    let mut result = String::new();

    // Build the "wearing" sentence.
    if nude {
        // Nude but has footwear/accessories.
        if !footwear.is_empty() {
            write!(
                result,
                "{name} is nude with {}",
                join_natural_with_articles(&footwear)
            )
            .unwrap();
        } else {
            write!(result, "{name} is nude").unwrap();
        }
    } else if topless || bottomless {
        let state = if topless { "topless" } else { "bottomless" };
        if !clothing.is_empty() {
            write!(
                result,
                "{name} is {state}, wearing {}",
                join_natural_with_articles(&clothing)
            )
            .unwrap();
            if !footwear.is_empty() {
                write!(
                    result,
                    " with {}",
                    join_natural_with_articles(&footwear)
                )
                .unwrap();
            } else if barefoot {
                write!(result, ", barefoot").unwrap();
            }
        } else if !footwear.is_empty() {
            write!(
                result,
                "{name} is {state} with {}",
                join_natural_with_articles(&footwear)
            )
            .unwrap();
        } else {
            write!(result, "{name} is {state}").unwrap();
            if barefoot {
                write!(result, " and barefoot").unwrap();
            }
        }
    } else if !clothing.is_empty() {
        write!(
            result,
            "{name} is wearing {}",
            join_natural_with_articles(&clothing)
        )
        .unwrap();
        if !footwear.is_empty() {
            write!(
                result,
                " with {}",
                join_natural_with_articles(&footwear)
            )
            .unwrap();
        } else if barefoot {
            write!(result, ", barefoot").unwrap();
        }
    } else if !footwear.is_empty() {
        write!(
            result,
            "{name} is wearing {}",
            join_natural_with_articles(&footwear)
        )
        .unwrap();
        if barefoot {
            // Has socks but no shoes.
            write!(result, ", barefoot").unwrap();
        }
    } else if barefoot {
        write!(result, "{name} is barefoot").unwrap();
    }

    if !result.is_empty() {
        result.push('.');
    }

    // Accessories sentence.
    if !accessories.is_empty() {
        if !result.is_empty() {
            result.push(' ');
        }
        write!(
            result,
            "{name} has {}.",
            join_natural_with_articles(&accessories)
        )
        .unwrap();
    }

    // Exposed body parts sentence.
    if result.is_empty() {
        result = format!("{name} is nude.");
    }
    append_exposed(&mut result, name, &exposed);

    result
}

/// Append an "exposed body parts" sentence if any are exposed.
fn append_exposed(result: &mut String, name: &str, exposed: &[&str]) {
    if exposed.is_empty() {
        return;
    }
    let parts: Vec<String> = exposed.iter().map(|s| s.to_string()).collect();
    write!(
        result,
        " {name}'s {} {} visible.",
        join_natural(&parts),
        if exposed.len() == 1 { "are" } else { "are" }
    )
    .unwrap();
}

/// Format an item name with an article ("a"/"an" for singular, none for plural).
fn with_article(name: &str) -> String {
    // Simple heuristic: names ending in 's' are treated as plural (no article).
    if name.ends_with('s') {
        name.to_string()
    } else {
        // Use "an" before vowel sounds.
        let first = name
            .chars()
            .next()
            .map(|c| c.to_ascii_lowercase())
            .unwrap_or('x');
        if matches!(first, 'a' | 'e' | 'i' | 'o' | 'u') {
            format!("an {name}")
        } else {
            format!("a {name}")
        }
    }
}

/// Join a list of item names with articles in natural English (Oxford comma).
///
/// - 1 item:  "a Crop Top"
/// - 2 items: "a Crop Top and a Mini Skirt"
/// - 3 items: "a Crop Top, a Mini Skirt, and Sneakers"
fn join_natural_with_articles(items: &[&str]) -> String {
    let with_articles: Vec<String> = items.iter().map(|s| with_article(s)).collect();
    join_natural(&with_articles)
}

/// Join strings in natural English with Oxford comma.
fn join_natural(items: &[String]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].clone(),
        2 => format!("{} and {}", items[0], items[1]),
        _ => {
            let (last, rest) = items.split_last().unwrap();
            let mut result = String::new();
            for (i, item) in rest.iter().enumerate() {
                if i > 0 {
                    result.push_str(", ");
                }
                result.push_str(item);
            }
            write!(result, ", and {last}").unwrap();
            result
        }
    }
}
