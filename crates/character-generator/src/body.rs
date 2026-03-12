use rand::Rng;
use std::fmt;

/// Body shape / build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyShape {
    // Child shapes (ages 3-11)
    Petite,
    Average,
    Sturdy,
    Lanky,

    // Teen / adult shapes (ages 12+)
    Slim,
    Athletic,
    Hourglass,
    Pear,
    Apple,
    Rectangle,
    Inverted,
    Curvy,
    PlusSized,
    Willowy,
    Stocky,
    Muscular,
    Voluptuous,
}

impl fmt::Display for BodyShape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BodyShape::Petite => "petite",
            BodyShape::Average => "average",
            BodyShape::Sturdy => "sturdy",
            BodyShape::Lanky => "lanky",
            BodyShape::Slim => "slim",
            BodyShape::Athletic => "athletic",
            BodyShape::Hourglass => "hourglass",
            BodyShape::Pear => "pear-shaped",
            BodyShape::Apple => "apple-shaped",
            BodyShape::Rectangle => "rectangle",
            BodyShape::Inverted => "inverted triangle",
            BodyShape::Curvy => "curvy",
            BodyShape::PlusSized => "plus-sized",
            BodyShape::Willowy => "willowy",
            BodyShape::Stocky => "stocky",
            BodyShape::Muscular => "muscular",
            BodyShape::Voluptuous => "voluptuous",
        };
        write!(f, "{s}")
    }
}

/// Breast size — only generated for teens/adults.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BreastSize {
    Flat,
    AAcup,
    Acup,
    Bcup,
    Ccup,
    Dcup,
    DDcup,
    Ecup,
    Fcup,
    Gcup,
}

impl fmt::Display for BreastSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BreastSize::Flat => "flat",
            BreastSize::AAcup => "AA-cup",
            BreastSize::Acup => "A-cup",
            BreastSize::Bcup => "B-cup",
            BreastSize::Ccup => "C-cup",
            BreastSize::Dcup => "D-cup",
            BreastSize::DDcup => "DD-cup",
            BreastSize::Ecup => "E-cup",
            BreastSize::Fcup => "F-cup",
            BreastSize::Gcup => "G-cup",
        };
        write!(f, "{s}")
    }
}

/// Height description appropriate for age.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Height {
    VeryShort,
    Short,
    BelowAverage,
    Average,
    AboveAverage,
    Tall,
    VeryTall,
}

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Height::VeryShort => "very short",
            Height::Short => "short",
            Height::BelowAverage => "below average height",
            Height::Average => "average height",
            Height::AboveAverage => "above average height",
            Height::Tall => "tall",
            Height::VeryTall => "very tall",
        };
        write!(f, "{s}")
    }
}

/// Pick weighted random.
fn weighted_pick<T: Copy>(rng: &mut impl Rng, options: &[(T, u32)]) -> T {
    let total: u32 = options.iter().map(|(_, w)| w).sum();
    let mut roll = rng.gen_range(0..total);
    for &(item, weight) in options {
        if roll < weight {
            return item;
        }
        roll -= weight;
    }
    options.last().unwrap().0
}

/// Generate a body shape based on age.
pub fn random_body_shape(rng: &mut impl Rng, age: u8) -> BodyShape {
    use BodyShape::*;

    let options: &[(BodyShape, u32)] = match age {
        3..=6 => &[
            (Petite, 30), (Average, 40), (Sturdy, 20), (Lanky, 10),
        ],
        7..=11 => &[
            (Petite, 20), (Average, 35), (Sturdy, 15), (Lanky, 20),
            (Athletic, 10),
        ],
        12..=14 => &[
            (Slim, 20), (Athletic, 15), (Rectangle, 20), (Pear, 10),
            (Willowy, 10), (Lanky, 10), (Petite, 10), (Curvy, 5),
        ],
        15..=17 => &[
            (Slim, 15), (Athletic, 15), (Hourglass, 10), (Pear, 10),
            (Rectangle, 10), (Curvy, 10), (Willowy, 10), (Petite, 8),
            (Stocky, 5), (Inverted, 5), (Muscular, 2),
        ],
        _ => &[
            (Slim, 10), (Athletic, 12), (Hourglass, 12), (Pear, 12),
            (Apple, 5), (Rectangle, 10), (Curvy, 12), (PlusSized, 5),
            (Willowy, 5), (Stocky, 5), (Muscular, 3), (Voluptuous, 5),
            (Inverted, 4),
        ],
    };

    weighted_pick(rng, options)
}

/// Generate a breast size based on age and body shape.
pub fn random_breast_size(rng: &mut impl Rng, age: u8, body: &BodyShape) -> BreastSize {
    use BreastSize::*;

    if age < 10 {
        return Flat;
    }

    if age <= 11 {
        return weighted_pick(rng, &[(Flat, 80), (AAcup, 20)]);
    }

    if age <= 13 {
        return weighted_pick(rng, &[
            (Flat, 20), (AAcup, 35), (Acup, 30), (Bcup, 15),
        ]);
    }

    if age <= 15 {
        return weighted_pick(rng, &[
            (AAcup, 10), (Acup, 25), (Bcup, 30), (Ccup, 20), (Dcup, 15),
        ]);
    }

    // 16+ — influenced by body shape
    match body {
        BodyShape::Slim | BodyShape::Willowy | BodyShape::Rectangle => {
            weighted_pick(rng, &[
                (AAcup, 10), (Acup, 25), (Bcup, 35), (Ccup, 20), (Dcup, 10),
            ])
        }
        BodyShape::Athletic | BodyShape::Muscular | BodyShape::Inverted => {
            weighted_pick(rng, &[
                (Acup, 10), (Bcup, 30), (Ccup, 35), (Dcup, 20), (DDcup, 5),
            ])
        }
        BodyShape::Curvy | BodyShape::Hourglass | BodyShape::Voluptuous => {
            weighted_pick(rng, &[
                (Bcup, 5), (Ccup, 20), (Dcup, 30), (DDcup, 25),
                (Ecup, 12), (Fcup, 5), (Gcup, 3),
            ])
        }
        BodyShape::PlusSized | BodyShape::Apple => {
            weighted_pick(rng, &[
                (Ccup, 10), (Dcup, 25), (DDcup, 30), (Ecup, 20),
                (Fcup, 10), (Gcup, 5),
            ])
        }
        BodyShape::Pear | BodyShape::Stocky => {
            weighted_pick(rng, &[
                (Acup, 5), (Bcup, 20), (Ccup, 30), (Dcup, 25),
                (DDcup, 15), (Ecup, 5),
            ])
        }
        BodyShape::Petite => {
            weighted_pick(rng, &[
                (AAcup, 15), (Acup, 35), (Bcup, 30), (Ccup, 15), (Dcup, 5),
            ])
        }
        _ => {
            weighted_pick(rng, &[
                (Acup, 10), (Bcup, 25), (Ccup, 30), (Dcup, 20),
                (DDcup, 10), (Ecup, 5),
            ])
        }
    }
}

/// Generate a height.
pub fn random_height(rng: &mut impl Rng) -> Height {
    use Height::*;
    weighted_pick(rng, &[
        (VeryShort, 5), (Short, 15), (BelowAverage, 15),
        (Average, 30), (AboveAverage, 15), (Tall, 15), (VeryTall, 5),
    ])
}
