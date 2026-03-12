use std::collections::HashMap;
use std::path::Path;

use crate::error::Error;
use crate::item::{Category, Item, ItemFile};
use crate::variation::{MatchFile, MatchSet, VariationCategory, VariationFile};

/// A collection of all available clothing items, variation categories,
/// and match sets, loaded from data files.
#[derive(Debug, Default)]
pub struct Wardrobe {
    items: Vec<Item>,
    /// Variation categories keyed by name (e.g. "fabric" → options).
    variation_categories: HashMap<String, VariationCategory>,
    /// Sets of variation values that look good together.
    match_sets: Vec<MatchSet>,
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
    ///     ...
    /// ```
    pub fn load_dir(&mut self, dir: &Path) -> Result<(), Error> {
        // Load variation categories first (from variations/ subdir).
        let var_dir = dir.join("variations");
        if var_dir.is_dir() {
            let entries = std::fs::read_dir(&var_dir)
                .map_err(|e| Error::Io(var_dir.to_path_buf(), e))?;
            for entry in entries {
                let entry = entry.map_err(|e| Error::Io(var_dir.to_path_buf(), e))?;
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "toml") {
                    self.load_variations_file(&path)?;
                }
            }
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

    /// All loaded variation categories.
    pub fn variation_categories(&self) -> &HashMap<String, VariationCategory> {
        &self.variation_categories
    }

    /// All loaded match sets.
    pub fn match_sets(&self) -> &[MatchSet] {
        &self.match_sets
    }

    /// Returns `true` if the wardrobe has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
