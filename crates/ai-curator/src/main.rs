mod client;
mod prompts;

use std::collections::HashMap;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use outfit_generator::{Item, ItemFile, Preset, Wardrobe};

use crate::client::AiConfig;

fn main() {
    let config = match AiConfig::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Configuration error: {e}");
            eprintln!();
            eprintln!("Required environment variables:");
            eprintln!("  AI_TOKEN    - API bearer token");
            eprintln!("Optional:");
            eprintln!("  AI_ENDPOINT - chat completions URL (default: OpenAI)");
            eprintln!("  AI_MODEL    - model name (default: gpt-4)");
            std::process::exit(1);
        }
    };

    let data_dir = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("data"));

    if !data_dir.is_dir() {
        eprintln!("Data directory not found: {}", data_dir.display());
        std::process::exit(1);
    }

    println!("Using endpoint: {}", config.endpoint);
    println!("Using model: {}", config.model);
    println!("Data directory: {}\n", data_dir.display());

    let mut wardrobe = Wardrobe::new();
    wardrobe.load_dir(&data_dir).expect("failed to load data directory");
    println!(
        "Loaded {} items, {} presets.\n",
        wardrobe.items().len(),
        wardrobe.presets().len(),
    );

    println!("Commands:");
    println!("  <prompt>                     - Generate a new preset from a description");
    println!("  adjust <PresetName> <instr>  - Adjust an existing preset");
    println!("  refresh                      - Update all existing presets with new items");
    println!("  quit                         - Exit\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        match input {
            "quit" | "exit" => break,
            "refresh" => {
                refresh_all_presets(&config, &mut wardrobe, &data_dir);
            }
            _ if input.starts_with("adjust ") => {
                let rest = &input["adjust ".len()..];
                // Find the preset name: try longest match against known presets.
                let mut found: Option<(String, String)> = None;
                for name in wardrobe.presets().keys() {
                    if let Some(remainder) = rest.strip_prefix(name.as_str()) {
                        let remainder = remainder.trim_start().to_string();
                        if found.as_ref().is_none_or(|(n, _)| n.len() < name.len()) {
                            found = Some((name.clone(), remainder));
                        }
                    }
                }
                match found {
                    Some((preset_name, instruction)) if !instruction.is_empty() => {
                        adjust_preset(&config, &mut wardrobe, &data_dir, &preset_name, &instruction);
                    }
                    Some((_, _)) => {
                        eprintln!("Usage: adjust <PresetName> <instruction>");
                    }
                    None => {
                        let available: Vec<_> = wardrobe.presets().keys().collect();
                        eprintln!("Unknown preset. Available: {available:?}");
                    }
                }
            }
            prompt => {
                generate_preset(&config, &mut wardrobe, &data_dir, prompt);
            }
        }
        println!();
    }
}

/// Generate a new preset from a user prompt.
fn generate_preset(config: &AiConfig, wardrobe: &mut Wardrobe, data_dir: &Path, user_prompt: &str) {
    println!("Generating preset for: \"{user_prompt}\"...\n");

    let system = prompts::system_prompt();
    let user_msg = prompts::generate_preset_prompt(wardrobe, user_prompt);

    let response_text = match client::chat_completion(config, &system, &user_msg) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("AI request failed: {e}");
            return;
        }
    };

    let response = match prompts::parse_generate_response(&response_text) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    // --- Write new items to category files ---
    let mut new_items_by_category: HashMap<String, Vec<Item>> = HashMap::new();
    let mut all_new_items: Vec<Item> = Vec::new();

    for new in &response.new_items {
        match prompts::new_item_to_item(new) {
            Ok(item) => {
                // Check for slug collision with existing items.
                if wardrobe.item_by_slug(&item.slug).is_some() {
                    println!("  Skipping {} (slug already exists)", item.slug);
                    continue;
                }
                let cat_key = new.category.clone();
                println!("  New item: {} ({})", item.name, item.slug);
                new_items_by_category
                    .entry(cat_key)
                    .or_default()
                    .push(item.clone());
                all_new_items.push(item);
            }
            Err(e) => {
                eprintln!("  Warning: {e}");
            }
        }
    }

    // Append new items to their category TOML files.
    for (category, items) in &new_items_by_category {
        let file_path = category_file_path(data_dir, category);
        append_items_to_file(&file_path, items);
    }

    // Add new items to wardrobe in memory.
    wardrobe.add_items(all_new_items);

    // --- Collect all slugs for the preset ---
    let mut preset_slugs: Vec<String> = response.matching_slugs.clone();
    for new in &response.new_items {
        if !preset_slugs.contains(&new.slug) {
            preset_slugs.push(new.slug.clone());
        }
    }

    // Verify slugs exist.
    preset_slugs.retain(|slug| {
        if wardrobe.item_by_slug(slug).is_some() {
            true
        } else {
            eprintln!("  Warning: slug \"{slug}\" not found in wardrobe, skipping");
            false
        }
    });

    preset_slugs.sort();
    preset_slugs.dedup();

    // --- Write preset file ---
    let preset = Preset {
        name: response.preset_name.clone(),
        prompt: Some(user_prompt.to_string()),
        items: preset_slugs,
    };

    let preset_path = data_dir
        .join("presets")
        .join(format!("{}.toml", response.preset_filename));

    write_preset_file(&preset_path, &preset);

    println!(
        "\nCreated preset \"{}\" with {} items at {}",
        preset.name,
        preset.items.len(),
        preset_path.display(),
    );
}

/// Adjust an existing preset based on a user instruction.
fn adjust_preset(
    config: &AiConfig,
    wardrobe: &mut Wardrobe,
    data_dir: &Path,
    preset_name: &str,
    instruction: &str,
) {
    let preset = match wardrobe.preset(preset_name) {
        Some(p) => p.clone(),
        None => {
            eprintln!("Preset \"{preset_name}\" not found.");
            return;
        }
    };

    println!("Adjusting \"{}\" with: \"{instruction}\"...\n", preset.name);

    let system = prompts::system_prompt();
    let user_msg = prompts::adjust_preset_prompt(wardrobe, &preset, instruction);

    let response_text = match client::chat_completion(config, &system, &user_msg) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("AI request failed: {e}");
            return;
        }
    };

    let response = match prompts::parse_adjust_response(&response_text) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("{e}");
            return;
        }
    };

    // --- Create new items ---
    let mut new_items_by_category: HashMap<String, Vec<Item>> = HashMap::new();
    let mut all_new_items: Vec<Item> = Vec::new();
    let mut new_slugs: Vec<String> = Vec::new();

    for new in &response.new_items {
        match prompts::new_item_to_item(new) {
            Ok(item) => {
                if wardrobe.item_by_slug(&item.slug).is_some() {
                    println!("  Skipping {} (slug already exists)", item.slug);
                    continue;
                }
                let cat_key = new.category.clone();
                println!("  New item: {} ({})", item.name, item.slug);
                new_items_by_category
                    .entry(cat_key)
                    .or_default()
                    .push(item.clone());
                new_slugs.push(item.slug.clone());
                all_new_items.push(item);
            }
            Err(e) => {
                eprintln!("  Warning: {e}");
            }
        }
    }

    // Append new items to category files.
    for (category, items) in &new_items_by_category {
        let file_path = category_file_path(data_dir, category);
        append_items_to_file(&file_path, items);
    }
    wardrobe.add_items(all_new_items);

    // --- Apply additions and removals ---
    let mut updated = preset.clone();

    // Remove slugs.
    for slug in &response.remove_slugs {
        if updated.items.contains(slug) {
            println!("  Removed: {slug}");
            updated.items.retain(|s| s != slug);
        }
    }

    // Add existing slugs.
    for slug in &response.add_slugs {
        if wardrobe.item_by_slug(slug).is_none() {
            eprintln!("  Warning: slug \"{slug}\" not found, skipping");
            continue;
        }
        if !updated.items.contains(slug) {
            println!("  Added: {slug}");
            updated.items.push(slug.clone());
        }
    }

    // Add newly created item slugs.
    for slug in &new_slugs {
        if !updated.items.contains(slug) {
            updated.items.push(slug.clone());
        }
    }

    updated.items.sort();
    updated.items.dedup();

    // Write updated preset.
    let filename = preset.name.to_lowercase().replace(' ', "_");
    let preset_path = data_dir
        .join("presets")
        .join(format!("{filename}.toml"));

    write_preset_file(&preset_path, &updated);

    println!(
        "\nUpdated \"{}\" ({} → {} items)",
        updated.name,
        preset.items.len(),
        updated.items.len(),
    );

    // Reload wardrobe.
    let mut new_wardrobe = Wardrobe::new();
    new_wardrobe.load_dir(data_dir).expect("failed to reload wardrobe");
    *wardrobe = new_wardrobe;
}

/// Refresh all existing presets by asking the AI if new items should be added.
fn refresh_all_presets(config: &AiConfig, wardrobe: &mut Wardrobe, data_dir: &Path) {
    let presets: Vec<Preset> = wardrobe
        .presets()
        .values()
        .cloned()
        .collect();

    if presets.is_empty() {
        println!("No presets to refresh.");
        return;
    }

    let system = prompts::system_prompt();

    for preset in &presets {
        // Only refresh presets that have a prompt.
        let prompt_text = match &preset.prompt {
            Some(p) => p.clone(),
            None => {
                println!("Skipping \"{}\" (no prompt stored)", preset.name);
                continue;
            }
        };

        println!("Refreshing \"{}\" (prompt: \"{}\")...", preset.name, prompt_text);

        let user_msg = prompts::refresh_preset_prompt(wardrobe, preset);

        let response_text = match client::chat_completion(config, &system, &user_msg) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("  AI request failed: {e}");
                continue;
            }
        };

        let response = match prompts::parse_refresh_response(&response_text) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("  {e}");
                continue;
            }
        };

        if response.add_slugs.is_empty() {
            println!("  No new items to add.");
            continue;
        }

        // Validate slugs exist.
        let valid_slugs: Vec<String> = response
            .add_slugs
            .into_iter()
            .filter(|slug| {
                if wardrobe.item_by_slug(slug).is_some() {
                    true
                } else {
                    eprintln!("  Warning: slug \"{slug}\" not found, skipping");
                    false
                }
            })
            .collect();

        if valid_slugs.is_empty() {
            println!("  No valid new items to add.");
            continue;
        }

        // Update preset.
        let mut updated = preset.clone();
        for slug in &valid_slugs {
            if !updated.items.contains(slug) {
                println!("  Adding: {slug}");
                updated.items.push(slug.clone());
            }
        }
        updated.items.sort();
        updated.items.dedup();

        // Derive filename from preset name.
        let filename = preset.name.to_lowercase().replace(' ', "_");
        let preset_path = data_dir
            .join("presets")
            .join(format!("{filename}.toml"));

        write_preset_file(&preset_path, &updated);
        println!(
            "  Updated \"{}\" ({} → {} items)",
            updated.name,
            preset.items.len(),
            updated.items.len(),
        );
    }

    // Reload wardrobe to pick up changes.
    let mut new_wardrobe = Wardrobe::new();
    new_wardrobe.load_dir(data_dir).expect("failed to reload wardrobe");
    *wardrobe = new_wardrobe;
}

/// Map a category name to the item TOML file it lives in.
fn category_file_path(data_dir: &Path, category: &str) -> PathBuf {
    let filename = match category {
        "top" => "tops",
        "bottom" => "bottoms",
        "dress" => "dresses",
        other => other,
    };
    data_dir.join(format!("{filename}.toml"))
}

/// Append items to an existing TOML file (or create it).
fn append_items_to_file(path: &Path, items: &[Item]) {
    // Read existing content.
    let mut existing = if path.exists() {
        let content = std::fs::read_to_string(path).expect("failed to read file");
        toml::from_str::<ItemFile>(&content)
            .unwrap_or_else(|_| ItemFile { items: vec![] })
    } else {
        ItemFile { items: vec![] }
    };

    existing.items.extend(items.iter().cloned());

    let content = toml::to_string_pretty(&existing).expect("failed to serialize items");
    std::fs::write(path, content).expect("failed to write file");
}

/// Write a preset to a TOML file.
fn write_preset_file(path: &Path, preset: &Preset) {
    let content = toml::to_string_pretty(preset).expect("failed to serialize preset");
    std::fs::write(path, content).expect("failed to write preset file");
}
