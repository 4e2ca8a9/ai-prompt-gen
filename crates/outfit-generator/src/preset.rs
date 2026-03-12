use serde::Deserialize;

/// A named list of item slugs that define a themed outfit subset.
///
/// When generating an outfit from a preset, only items whose slugs appear
/// in the preset's list are considered as candidates.
#[derive(Debug, Clone, Deserialize)]
pub struct Preset {
    pub name: String,
    pub items: Vec<String>,
}

/// TOML file for a preset.
///
/// ```toml
/// name = "Cheerleader"
/// items = [
///     "top.cheer_shell",
///     "bottom.cheer_skirt",
///     "shoes.cheer_shoes",
/// ]
/// ```
pub type PresetFile = Preset;
