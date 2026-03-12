use std::path::Path;

use outfit_generator::{generate_outfit, Wardrobe};

fn main() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");

    let mut wardrobe = Wardrobe::new();
    wardrobe
        .load_dir(&data_dir)
        .expect("failed to load data directory");

    println!(
        "Wardrobe: {} items, {} variation categories, {} match sets.\n",
        wardrobe.items().len(),
        wardrobe.variation_categories().len(),
        wardrobe.match_sets().len(),
    );

    let outfit = generate_outfit(&wardrobe);

    if outfit.is_empty() {
        println!("Generated outfit: nude!");
    } else {
        println!("Generated outfit:");
        println!("{outfit}");
    }
}
