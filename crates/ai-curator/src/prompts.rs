use outfit_generator::{Item, Preset, Wardrobe};
use serde::{Deserialize, Serialize};

/// Build the system prompt that teaches the AI about our data model.
pub fn system_prompt() -> String {
    r#"You are a fashion curator for an outfit generation system.

The system has clothing items organized by category. Each item has:
- slug: unique ID like "category.snake_case_name" (e.g. "top.crop_top")
- name: display name (e.g. "Crop Top")
- category: one of: underwear, top, bottom, dress, outerwear, accessory, jewelry, socks, shoes
- slots: body slots the item occupies. Two items conflict if they share a slot.
  Valid slots: head, hair, ears, eyes, neck, torso_under, torso, torso_outer,
  crotch, legs_under, legs, lower_legs, waist, wrist_left, wrist_right,
  fingers, ankles, feet_inner, feet_outer, bag
- variations: which variation categories apply. Current categories: "fabric", "metal", "leather"
  - Clothing items typically use "fabric"
  - Jewelry and metal accessories use "metal"
  - Leather goods (shoes, bags, belts, jackets) use "leather"

You MUST respond with valid JSON only, no markdown fences, no explanation text.
"#
    .into()
}

/// Build the user prompt for generating a new preset.
pub fn generate_preset_prompt(wardrobe: &Wardrobe, user_prompt: &str) -> String {
    let items_json = serde_json::to_string_pretty(wardrobe.items()).unwrap();

    format!(
        r#"Here are all existing items in the wardrobe:

{items_json}

The user wants a preset for: "{user_prompt}"

Respond with a JSON object:
{{
  "preset_name": "Human Readable Name",
  "preset_filename": "snake_case_filename",
  "matching_slugs": ["slug1", "slug2"],
  "new_items": [
    {{
      "slug": "category.snake_case_name",
      "name": "Display Name",
      "category": "category",
      "slots": ["slot1"],
      "variations": ["fabric"]
    }}
  ]
}}

Rules:
- "matching_slugs" should list ALL existing item slugs that fit this theme
- "new_items" should contain new items that don't exist yet but would fit the theme
- New item slugs must not collide with existing slugs
- Choose appropriate slots and variations for new items
- Be thorough: include underwear, socks, shoes, accessories, not just the obvious pieces
- Only create items that are distinct from existing ones"#
    )
}

/// Build the user prompt for refreshing an existing preset with new items.
pub fn refresh_preset_prompt(wardrobe: &Wardrobe, preset: &Preset) -> String {
    let items_json = serde_json::to_string_pretty(wardrobe.items()).unwrap();
    let current_slugs = serde_json::to_string_pretty(&preset.items).unwrap();
    let prompt_text = preset.prompt.as_deref().unwrap_or(&preset.name);

    format!(
        r#"Here are all items in the wardrobe:

{items_json}

Here is an existing preset called "{name}" (original prompt: "{prompt_text}").
Its current items are:

{current_slugs}

Check if any items in the wardrobe (that are NOT already in the preset) should be added to this preset based on the original prompt.

Respond with a JSON object:
{{
  "add_slugs": ["slug1", "slug2"]
}}

"add_slugs" should list only NEW slugs to add (not already in the preset).
If no items should be added, return {{"add_slugs": []}}."#,
        name = preset.name,
    )
}

/// Parsed response from the AI for preset generation.
#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub preset_name: String,
    pub preset_filename: String,
    pub matching_slugs: Vec<String>,
    pub new_items: Vec<NewItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewItem {
    pub slug: String,
    pub name: String,
    pub category: String,
    pub slots: Vec<String>,
    #[serde(default)]
    pub variations: Vec<String>,
}

/// Parsed response from the AI for preset refresh.
#[derive(Debug, Deserialize)]
pub struct RefreshResponse {
    pub add_slugs: Vec<String>,
}

/// Build the user prompt for adjusting an existing preset.
pub fn adjust_preset_prompt(wardrobe: &Wardrobe, preset: &Preset, instruction: &str) -> String {
    let items_json = serde_json::to_string_pretty(wardrobe.items()).unwrap();
    let current_slugs = serde_json::to_string_pretty(&preset.items).unwrap();
    let prompt_text = preset.prompt.as_deref().unwrap_or(&preset.name);

    format!(
        r#"Here are all items in the wardrobe:

{items_json}

Here is an existing preset called "{name}" (original prompt: "{prompt_text}").
Its current items are:

{current_slugs}

The user wants to adjust this preset with the following instruction:
"{instruction}"

Respond with a JSON object:
{{
  "add_slugs": ["slug1", "slug2"],
  "remove_slugs": ["slug3"],
  "new_items": [
    {{
      "slug": "category.snake_case_name",
      "name": "Display Name",
      "category": "category",
      "slots": ["slot1"],
      "variations": ["fabric"]
    }}
  ]
}}

Rules:
- "add_slugs" lists existing wardrobe item slugs to ADD to the preset (not already in it)
- "remove_slugs" lists slugs to REMOVE from the preset
- "new_items" lists brand-new items to create and add (slugs must not collide with existing items)
- New items should have appropriate slots and variations
- Use empty arrays for any field that doesn't apply
- Follow the user's instruction precisely"#,
        name = preset.name,
    )
}

/// Parsed response from the AI for preset adjustment.
#[derive(Debug, Deserialize)]
pub struct AdjustResponse {
    #[serde(default)]
    pub add_slugs: Vec<String>,
    #[serde(default)]
    pub remove_slugs: Vec<String>,
    #[serde(default)]
    pub new_items: Vec<NewItem>,
}

/// Try to parse an AdjustResponse from the AI's text output.
pub fn parse_adjust_response(text: &str) -> Result<AdjustResponse, String> {
    let json = extract_json(text);
    serde_json::from_str(json).map_err(|e| format!("Failed to parse AI response: {e}\n\nRaw:\n{text}"))
}

/// Try to parse a GenerateResponse from the AI's text output.
/// Handles cases where the AI wraps JSON in markdown fences.
pub fn parse_generate_response(text: &str) -> Result<GenerateResponse, String> {
    let json = extract_json(text);
    serde_json::from_str(json).map_err(|e| format!("Failed to parse AI response: {e}\n\nRaw:\n{text}"))
}

/// Try to parse a RefreshResponse from the AI's text output.
pub fn parse_refresh_response(text: &str) -> Result<RefreshResponse, String> {
    let json = extract_json(text);
    serde_json::from_str(json).map_err(|e| format!("Failed to parse AI response: {e}\n\nRaw:\n{text}"))
}

/// Strip markdown code fences if present.
fn extract_json(text: &str) -> &str {
    let trimmed = text.trim();
    if let Some(rest) = trimmed.strip_prefix("```json") {
        rest.strip_suffix("```").unwrap_or(rest).trim()
    } else if let Some(rest) = trimmed.strip_prefix("```") {
        rest.strip_suffix("```").unwrap_or(rest).trim()
    } else {
        trimmed
    }
}

/// Convert a NewItem into an outfit_generator::Item.
pub fn new_item_to_item(new: &NewItem) -> Result<Item, String> {
    let toml_str = format!(
        r#"slug = {slug}
name = {name}
category = {category}
slots = {slots}
variations = {variations}
"#,
        slug = toml_value(&new.slug),
        name = toml_value(&new.name),
        category = toml_value(&new.category),
        slots = toml_array(&new.slots),
        variations = toml_array(&new.variations),
    );
    toml::from_str::<Item>(&toml_str)
        .map_err(|e| format!("Invalid item from AI ({}): {e}", new.slug))
}

fn toml_value(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn toml_array(v: &[String]) -> String {
    let inner: Vec<String> = v.iter().map(|s| toml_value(s)).collect();
    format!("[{}]", inner.join(", "))
}
