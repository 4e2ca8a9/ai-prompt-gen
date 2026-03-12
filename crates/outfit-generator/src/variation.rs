use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;

/// A named variation category (e.g. "fabric") and its possible values.
#[derive(Debug, Clone)]
pub struct VariationCategory {
    pub name: String,
    pub options: Vec<String>,
}

/// TOML file for defining a variation category.
///
/// ```toml
/// category = "fabric"
///
/// [[options]]
/// name = "Black Silk"
///
/// [[options]]
/// name = "White Cotton"
/// ```
#[derive(Debug, Deserialize)]
pub struct VariationFile {
    pub category: String,
    pub options: Vec<VariationOption>,
}

#[derive(Debug, Deserialize)]
pub struct VariationOption {
    pub name: String,
}

/// A named set of variation values that look good together.
///
/// Each variation category maps to a list of compatible options.
/// During outfit generation, one is chosen at random per item.
#[derive(Debug, Clone, Deserialize)]
pub struct MatchSet {
    pub name: String,
    pub set: HashMap<String, Vec<String>>,
}

/// TOML file containing match sets.
///
/// ```toml
/// [[matches]]
/// name = "Dark & Gold"
/// [matches.set]
/// fabric = ["Black Silk", "Black Denim", "Charcoal Wool"]
/// metal = ["Yellow Gold", "Brushed Gold"]
/// leather = ["Black Leather", "Patent Black"]
/// ```
#[derive(Debug, Deserialize)]
pub struct MatchFile {
    pub matches: Vec<MatchSet>,
}

/// An item in a generated outfit, with its assigned variations.
#[derive(Debug, Clone)]
pub struct OutfitItem {
    pub slug: String,
    pub name: String,
    /// Maps variation category → chosen value (e.g. "fabric" → "Black Silk").
    pub variations: HashMap<String, String>,
}

impl fmt::Display for OutfitItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.variations.is_empty() {
            let mut parts: Vec<_> = self.variations.iter().collect();
            parts.sort_by_key(|(k, _)| *k);
            let desc: Vec<_> = parts.iter().map(|(k, v)| format!("{k}: {v}")).collect();
            write!(f, " ({})", desc.join(", "))?;
        }
        Ok(())
    }
}
