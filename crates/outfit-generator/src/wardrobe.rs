use std::path::Path;

use crate::error::Error;
use crate::item::{Category, Item, ItemFile};

/// A collection of all available clothing and accessory items,
/// loaded from one or more TOML data files.
#[derive(Debug, Default)]
pub struct Wardrobe {
    items: Vec<Item>,
}

impl Wardrobe {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load items from a single TOML file and add them to the wardrobe.
    pub fn load_file(&mut self, path: &Path) -> Result<(), Error> {
        let contents = std::fs::read_to_string(path)
            .map_err(|e| Error::Io(path.to_path_buf(), e))?;
        let file: ItemFile = toml::from_str(&contents)
            .map_err(|e| Error::Parse(path.to_path_buf(), e))?;
        self.items.extend(file.items);
        Ok(())
    }

    /// Load every `.toml` file in a directory (non-recursive).
    pub fn load_dir(&mut self, dir: &Path) -> Result<(), Error> {
        let entries = std::fs::read_dir(dir)
            .map_err(|e| Error::Io(dir.to_path_buf(), e))?;
        for entry in entries {
            let entry = entry.map_err(|e| Error::Io(dir.to_path_buf(), e))?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "toml") {
                self.load_file(&path)?;
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

    /// Returns `true` if the wardrobe is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
