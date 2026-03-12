use std::fmt;

use rand::Rng;

use crate::body::{self, BodyShape, BreastSize, Height};
use crate::ethnicity::Ethnicity;
use crate::features::{self, EyeColor, SkinTone};
use crate::hair::{self, HairColor, HairLength, HairStyle, HairTexture};

/// A fully generated female character.
#[derive(Debug, Clone)]
pub struct Character {
    pub age: u8,
    pub ethnicity: Ethnicity,
    pub skin_tone: SkinTone,
    pub eye_color: EyeColor,
    pub hair_color: HairColor,
    pub hair_texture: HairTexture,
    pub hair_length: HairLength,
    pub hair_style: HairStyle,
    pub body_shape: BodyShape,
    pub breast_size: BreastSize,
    pub height: Height,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let age_desc = match self.age {
            3..=5 => "toddler girl",
            6..=9 => "young girl",
            10..=12 => "preteen girl",
            13..=17 => "teenage girl",
            18..=25 => "young woman",
            26..=35 => "woman",
            36..=55 => "middle-aged woman",
            _ => "older woman",
        };

        writeln!(f, "{} year old {} ({})", self.age, age_desc, self.ethnicity)?;
        writeln!(f, "  Skin: {}", self.skin_tone)?;
        writeln!(f, "  Eyes: {}", self.eye_color)?;
        writeln!(f, "  Hair: {} {} {} hair, {}", self.hair_color, self.hair_length, self.hair_texture, self.hair_style)?;
        writeln!(f, "  Body: {}, {}", self.body_shape, self.height)?;
        if self.age >= 10 {
            writeln!(f, "  Bust: {}", self.breast_size)?;
        }
        Ok(())
    }
}

impl Character {
    /// Generate a text description suitable for use as a prompt.
    pub fn describe(&self, name: &str) -> String {
        let age_desc = match self.age {
            3..=5 => "toddler girl",
            6..=9 => "young girl",
            10..=12 => "preteen girl",
            13..=17 => "teenage girl",
            18..=25 => "young woman",
            26..=35 => "woman",
            36..=55 => "middle-aged woman",
            _ => "older woman",
        };

        let mut parts = Vec::new();

        parts.push(format!(
            "{name} is a {age} year old {eth} {desc}",
            age = self.age,
            eth = self.ethnicity,
            desc = age_desc,
        ));

        parts.push(format!(
            "with {} skin, {} eyes",
            self.skin_tone, self.eye_color,
        ));

        parts.push(format!(
            "and {} {} {} hair worn {}",
            self.hair_color, self.hair_length, self.hair_texture, self.hair_style,
        ));

        let mut body_parts = vec![format!(
            "She has a {} build and is {}",
            self.body_shape, self.height,
        )];

        if self.age >= 12 {
            body_parts.push(format!("with a {} bust", self.breast_size));
        }

        format!("{}, {}. {}.", parts.join(", "), parts.len(), body_parts.join(", "))
    }

    /// Shorter description.
    pub fn short_describe(&self, name: &str) -> String {
        let mut s = format!(
            "{name} is a {} year old {} girl with {} skin, {} eyes, and {} {} hair worn {}.",
            self.age, self.ethnicity, self.skin_tone, self.eye_color,
            self.hair_color, self.hair_texture, self.hair_style,
        );
        s.push_str(&format!(
            " She is {} with a {} build",
            self.height, self.body_shape,
        ));
        if self.age >= 12 {
            s.push_str(&format!(" and a {} bust", self.breast_size));
        }
        s.push('.');
        s
    }
}

/// Generate a random character with the given age.
/// Ethnicity is chosen randomly.
pub fn generate_character(age: u8) -> Character {
    let mut rng = rand::thread_rng();
    generate_character_with_rng(&mut rng, age, None)
}

/// Generate a character with a specific ethnicity.
pub fn generate_character_with_ethnicity(age: u8, ethnicity: Ethnicity) -> Character {
    let mut rng = rand::thread_rng();
    generate_character_with_rng(&mut rng, age, Some(ethnicity))
}

/// Core generation with explicit RNG.
pub fn generate_character_with_rng(
    rng: &mut impl Rng,
    age: u8,
    ethnicity: Option<Ethnicity>,
) -> Character {
    let ethnicity = ethnicity.unwrap_or_else(|| Ethnicity::random(rng));
    let skin_tone = features::random_skin_tone(rng, &ethnicity);
    let eye_color = features::random_eye_color(rng, &ethnicity);
    let hair_color = hair::random_hair_color(rng, &ethnicity);
    let hair_texture = hair::random_hair_texture(rng, &ethnicity);
    let hair_length = hair::random_hair_length(rng, age);
    let hair_style = hair::random_hair_style(rng, age, &hair_texture, &hair_length);
    let body_shape = body::random_body_shape(rng, age);
    let breast_size = body::random_breast_size(rng, age, &body_shape);
    let height = body::random_height(rng);

    Character {
        age,
        ethnicity,
        skin_tone,
        eye_color,
        hair_color,
        hair_texture,
        hair_length,
        hair_style,
        body_shape,
        breast_size,
        height,
    }
}
