use std::path::Path;

use outfit_generator::{generate_outfit, generate_outfit_from_preset, Wardrobe};

fn main() {
    let data_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data");

    let mut wardrobe = Wardrobe::new();
    wardrobe
        .load_dir(&data_dir)
        .expect("failed to load data directory");

    println!(
        "Wardrobe: {} items, {} variation categories, {} match sets, {} presets.\n",
        wardrobe.items().len(),
        wardrobe.variation_categories().len(),
        wardrobe.match_sets().len(),
        wardrobe.presets().len(),
    );

    // Check if a preset name was passed as an argument (supports spaces).
    let args: Vec<String> = std::env::args().skip(1).collect();
    let preset_name = if args.is_empty() {
        None
    } else {
        Some(args.join(" "))
    };

    let outfit = if let Some(ref name) = preset_name {
        let preset = wardrobe
            .preset(name)
            .unwrap_or_else(|| {
                let available: Vec<_> = wardrobe.presets().keys().collect();
                eprintln!("Unknown preset \"{name}\". Available: {available:?}");
                std::process::exit(1);
            });
        println!("Using preset: {}\n", preset.name);
        generate_outfit_from_preset(&wardrobe, preset)
    } else {
        generate_outfit(&wardrobe)
    };

    if outfit.is_empty() {
        println!("Generated outfit: nude!");
    } else {
        println!("Generated outfit:");
        println!("{outfit}");
    }
}
