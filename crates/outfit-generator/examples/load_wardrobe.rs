use std::path::Path;

use outfit_generator::{Category, Wardrobe};

fn main() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");

    let mut wardrobe = Wardrobe::new();
    wardrobe.load_dir(&data_dir).expect("failed to load data directory");

    println!("Loaded {} items total.\n", wardrobe.items().len());

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
            println!("  - {} (slots: {:?})", item.name, item.slots);
        }
        println!();
    }
}
