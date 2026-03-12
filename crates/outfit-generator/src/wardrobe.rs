use std::collections::HashMap;
use std::path::Path;

use crate::error::Error;
use crate::item::{Category, Item, ItemFile};
use crate::preset::{Preset, PresetFile};
use crate::variation::{MatchFile, MatchSet, VariationCategory, VariationFile};

/// A collection of all available clothing items, variation categories,
/// match sets, and presets, loaded from data files.
#[derive(Debug, Default)]
pub struct Wardrobe {
    items: Vec<Item>,
    /// Variation categories keyed by name (e.g. "fabric" → options).
    variation_categories: HashMap<String, VariationCategory>,
    /// Sets of variation values that look good together.
    match_sets: Vec<MatchSet>,
    /// Named presets keyed by name.
    presets: HashMap<String, Preset>,
}

impl Wardrobe {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load items from a single TOML file.
    pub fn load_items_file(&mut self, path: &Path) -> Result<(), Error> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| Error::Io(path.to_path_buf(), e))?;
        let file: ItemFile = toml::from_str(&contents)
            .map_err(|e| Error::Parse(path.to_path_buf(), e))?;
        self.items.extend(file.items);
        Ok(())
    }

    /// Load a variation category from a TOML file.
    pub fn load_variations_file(&mut self, path: &Path) -> Result<(), Error> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| Error::Io(path.to_path_buf(), e))?;
        let file: VariationFile = toml::from_str(&contents)
            .map_err(|e| Error::Parse(path.to_path_buf(), e))?;
        let cat = VariationCategory {
            name: file.category.clone(),
            options: file.options.into_iter().map(|o| o.name).collect(),
        };
        self.variation_categories.insert(file.category, cat);
        Ok(())
    }

    /// Load match sets from a TOML file.
    pub fn load_matches_file(&mut self, path: &Path) -> Result<(), Error> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| Error::Io(path.to_path_buf(), e))?;
        let file: MatchFile = toml::from_str(&contents)
            .map_err(|e| Error::Parse(path.to_path_buf(), e))?;
        self.match_sets.extend(file.matches);
        Ok(())
    }

    /// Load a preset from a TOML file.
    pub fn load_preset_file(&mut self, path: &Path) -> Result<(), Error> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| Error::Io(path.to_path_buf(), e))?;
        let preset: PresetFile = toml::from_str(&contents)
            .map_err(|e| Error::Parse(path.to_path_buf(), e))?;
        self.presets.insert(preset.name.clone(), preset);
        Ok(())
    }

    /// Load an entire data directory.
    ///
    /// Expects the layout:
    /// ```text
    /// dir/
    ///   underwear.toml      # item files (any .toml in root)
    ///   tops.toml
    ///   ...
    ///   matches.toml         # match sets
    ///   variations/          # variation category files
    ///     fabric.toml
    ///     metal.toml
    ///   presets/             # preset files
    ///     cheerleader.toml
    /// ```
    pub fn load_dir(&mut self, dir: &Path) -> Result<(), Error> {
        // Load variation categories (from variations/ subdir).
        let var_dir = dir.join("variations");
        if var_dir.is_dir() {
            load_toml_dir(&var_dir, |path| self.load_variations_file(path))?;
        }

        // Load presets (from presets/ subdir).
        let preset_dir = dir.join("presets");
        if preset_dir.is_dir() {
            load_toml_dir(&preset_dir, |path| self.load_preset_file(path))?;
        }

        // Load top-level .toml files (items + matches).
        let entries = std::fs::read_dir(dir)
            .map_err(|e| Error::Io(dir.to_path_buf(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| Error::Io(dir.to_path_buf(), e))?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if path.extension().is_some_and(|ext| ext == "toml") {
                if path.file_stem().is_some_and(|s| s == "matches") {
                    self.load_matches_file(&path)?;
                } else {
                    self.load_items_file(&path)?;
                }
            }
        }

        Ok(())
    }

    /// All items in the wardrobe.
    pub fn items(&self) -> &[Item] {
        &self.items
    }

    /// All items belonging to a specific category.
    pub fn items_in(&self, category: Category) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.category == category)
            .collect()
    }

    /// Look up an item by its slug.
    pub fn item_by_slug(&self, slug: &str) -> Option<&Item> {
        self.items.iter().find(|item| item.slug == slug)
    }

    /// Get a preset by name.
    pub fn preset(&self, name: &str) -> Option<&Preset> {
        self.presets.get(name)
    }

    /// All loaded presets.
    pub fn presets(&self) -> &HashMap<String, Preset> {
        &self.presets
    }

    /// All loaded variation categories.
    pub fn variation_categories(&self) -> &HashMap<String, VariationCategory> {
        &self.variation_categories
    }

    /// All loaded match sets.
    pub fn match_sets(&self) -> &[MatchSet] {
        &self.match_sets
    }

    /// Add items programmatically.
    pub fn add_items(&mut self, items: Vec<Item>) {
        self.items.extend(items);
    }

    /// Returns `true` if the wardrobe has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Helper: iterate all `.toml` files in a directory and call `f` on each.
fn load_toml_dir(dir: &Path, mut f: impl FnMut(&Path) -> Result<(), Error>) -> Result<(), Error> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| Error::Io(dir.to_path_buf(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| Error::Io(dir.to_path_buf(), e))?;
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "toml") {
            f(&path)?;
        }
    }
    Ok(())
}
