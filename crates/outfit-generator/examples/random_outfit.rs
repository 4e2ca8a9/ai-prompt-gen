use std::collections::HashSet;
use std::path::Path;

use outfit_generator::{
    describe_outfit, generate_outfit, generate_outfit_from_preset, Slot, Wardrobe,
};

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

    let omit_slots = HashSet::new();

    let outfit = if let Some(ref name) = preset_name {
        let preset = wardrobe.preset(name).unwrap_or_else(|| {
            let available: Vec<_> = wardrobe.presets().keys().collect();
            eprintln!("Unknown preset \"{name}\". Available: {available:?}");
            std::process::exit(1);
        });
        println!("Using preset: {}\n", preset.name);
        generate_outfit_from_preset(&wardrobe, preset, &omit_slots)
    } else {
        generate_outfit(&wardrobe, &omit_slots)
    };

    // All body parts visible for this example.
    let visible: HashSet<Slot> = HashSet::from([
        Slot::Head,
        Slot::Hair,
        Slot::Ears,
        Slot::Eyes,
        Slot::Neck,
        Slot::TorsoUnder,
        Slot::Torso,
        Slot::TorsoOuter,
        Slot::Crotch,
        Slot::LegsUnder,
        Slot::Legs,
        Slot::LowerLegs,
        Slot::Waist,
        Slot::WristLeft,
        Slot::WristRight,
        Slot::Fingers,
        Slot::Ankles,
        Slot::FeetInner,
        Slot::FeetOuter,
        Slot::Bag,
    ]);

    println!("Generated outfit:");
    println!("{outfit}");
    println!();
    println!("{}", describe_outfit("Amy", &outfit, &wardrobe, &visible));
}
