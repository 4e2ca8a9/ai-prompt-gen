use std::path::Path;

use outfit_generator::{Category, Wardrobe};

fn main() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");

    let mut wardrobe = Wardrobe::new();
    wardrobe.load_dir(&data_dir).expect("failed to load data directory");

    println!("Loaded {} items total.", wardrobe.items().len());
    println!(
        "Loaded {} variation categories: {:?}",
        wardrobe.variation_categories().len(),
        wardrobe.variation_categories().keys().collect::<Vec<_>>(),
    );
    println!("Loaded {} match sets.\n", wardrobe.match_sets().len());

    let categories = [
        Category::Underwear,
        Category::Top,
        Category::Bottom,
        Category::Dress,
        Category::Outerwear,
        Category::Socks,
        Category::Shoes,
        Category::Accessory,
        Category::Jewelry,
    ];

    for cat in categories {
        let items = wardrobe.items_in(cat);
        println!("{cat:?} ({} items):", items.len());
        for item in &items {
            if item.variations.is_empty() {
                println!("  - {} (slots: {:?})", item.name, item.slots);
            } else {
                println!(
                    "  - {} (slots: {:?}, variations: {:?})",
                    item.name, item.slots, item.variations
                );
            }
        }
        println!();
    }

    println!("Match sets:");
    for ms in wardrobe.match_sets() {
        println!("  {:?}: {:?}", ms.name, ms.set);
    }
}
