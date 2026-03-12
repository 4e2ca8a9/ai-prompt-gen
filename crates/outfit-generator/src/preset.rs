use serde::{Deserialize, Serialize};

/// A named list of item slugs that define a themed outfit subset.
///
/// When generating an outfit from a preset, only items whose slugs appear
/// in the preset's list are considered as candidates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    /// The original prompt used to generate this preset (if AI-generated).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    pub items: Vec<String>,
}

/// TOML file for a preset.
///
/// ```toml
/// name = "European Schoolgirl"
/// prompt = "european schoolgirl"
/// items = [
///     "top.peter_pan_collar_blouse",
///     "bottom.pleated_midi_skirt",
///     "shoes.loafers",
/// ]
/// ```
pub type PresetFile = Preset;
