use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt;

use crate::ethnicity::{EthnicGroup, Ethnicity};

/// Natural hair color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HairColor {
    Platinum,
    Flaxen,
    GoldenBlonde,
    StrawberryBlonde,
    HoneyBlonde,
    SandyBlonde,
    AshBlonde,
    LightBrown,
    MediumBrown,
    ChestnutBrown,
    DarkBrown,
    VeryDarkBrown,
    Black,
    JetBlack,
    Auburn,
    Copper,
    Ginger,
    Red,
    DarkAuburn,
    SilverGrey,
    White,
}

impl fmt::Display for HairColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HairColor::Platinum => "platinum blonde",
            HairColor::Flaxen => "flaxen",
            HairColor::GoldenBlonde => "golden blonde",
            HairColor::StrawberryBlonde => "strawberry blonde",
            HairColor::HoneyBlonde => "honey blonde",
            HairColor::SandyBlonde => "sandy blonde",
            HairColor::AshBlonde => "ash blonde",
            HairColor::LightBrown => "light brown",
            HairColor::MediumBrown => "medium brown",
            HairColor::ChestnutBrown => "chestnut brown",
            HairColor::DarkBrown => "dark brown",
            HairColor::VeryDarkBrown => "very dark brown",
            HairColor::Black => "black",
            HairColor::JetBlack => "jet black",
            HairColor::Auburn => "auburn",
            HairColor::Copper => "copper",
            HairColor::Ginger => "ginger",
            HairColor::Red => "red",
            HairColor::DarkAuburn => "dark auburn",
            HairColor::SilverGrey => "silver-grey",
            HairColor::White => "white",
        };
        write!(f, "{s}")
    }
}

/// Hair texture / type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HairTexture {
    Straight,
    Wavy,
    Curly,
    Coily,
    Kinky,
}

impl fmt::Display for HairTexture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HairTexture::Straight => "straight",
            HairTexture::Wavy => "wavy",
            HairTexture::Curly => "curly",
            HairTexture::Coily => "coily",
            HairTexture::Kinky => "kinky",
        };
        write!(f, "{s}")
    }
}

/// Hair length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HairLength {
    Shaved,
    Buzzed,
    VeryShort,
    Short,
    ChinLength,
    ShoulderLength,
    MidBack,
    WaistLength,
    HipLength,
}

impl fmt::Display for HairLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            HairLength::Shaved => "shaved",
            HairLength::Buzzed => "buzzed",
            HairLength::VeryShort => "very short",
            HairLength::Short => "short",
            HairLength::ChinLength => "chin-length",
            HairLength::ShoulderLength => "shoulder-length",
            HairLength::MidBack => "mid-back length",
            HairLength::WaistLength => "waist-length",
            HairLength::HipLength => "hip-length",
        };
        write!(f, "{s}")
    }
}

/// A specific hair style.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HairStyle {
    pub name: String,
}

impl fmt::Display for HairStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

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

/// Generate a hair color appropriate for the given ethnicity.
pub fn random_hair_color(rng: &mut impl Rng, ethnicity: &Ethnicity) -> HairColor {
    use HairColor::*;

    let options: &[(HairColor, u32)] = match ethnicity.group() {
        EthnicGroup::EastAsian => &[
            (Black, 40), (JetBlack, 30), (VeryDarkBrown, 20), (DarkBrown, 10),
        ],
        EthnicGroup::SoutheastAsian => &[
            (Black, 40), (JetBlack, 25), (VeryDarkBrown, 20), (DarkBrown, 15),
        ],
        EthnicGroup::SouthAsian => &[
            (Black, 35), (JetBlack, 25), (VeryDarkBrown, 20),
            (DarkBrown, 12), (DarkAuburn, 5), (ChestnutBrown, 3),
        ],
        EthnicGroup::CentralAsian => &[
            (Black, 25), (JetBlack, 15), (VeryDarkBrown, 25),
            (DarkBrown, 20), (ChestnutBrown, 10), (MediumBrown, 5),
        ],
        EthnicGroup::MiddleEastern => &[
            (Black, 25), (JetBlack, 15), (VeryDarkBrown, 25),
            (DarkBrown, 15), (DarkAuburn, 8), (ChestnutBrown, 7),
            (MediumBrown, 5),
        ],
        EthnicGroup::European => match ethnicity {
            Ethnicity::Scandinavian => &[
                (Platinum, 8), (Flaxen, 10), (GoldenBlonde, 15),
                (AshBlonde, 12), (SandyBlonde, 12), (HoneyBlonde, 8),
                (StrawberryBlonde, 5), (LightBrown, 15), (MediumBrown, 10),
                (DarkBrown, 3), (Red, 2),
            ],
            Ethnicity::Celtic => &[
                (Red, 10), (Ginger, 8), (Copper, 8), (Auburn, 8),
                (StrawberryBlonde, 6), (GoldenBlonde, 5), (AshBlonde, 5),
                (LightBrown, 12), (MediumBrown, 12), (DarkBrown, 10),
                (Black, 5), (ChestnutBrown, 8), (DarkAuburn, 3),
            ],
            Ethnicity::SouthernEuropean => &[
                (Black, 10), (JetBlack, 5), (VeryDarkBrown, 20),
                (DarkBrown, 25), (ChestnutBrown, 15), (MediumBrown, 12),
                (DarkAuburn, 5), (Auburn, 3), (LightBrown, 5),
            ],
            _ => &[
                (Platinum, 3), (Flaxen, 4), (GoldenBlonde, 8),
                (AshBlonde, 8), (SandyBlonde, 8), (HoneyBlonde, 5),
                (StrawberryBlonde, 3), (LightBrown, 12), (MediumBrown, 15),
                (ChestnutBrown, 10), (DarkBrown, 12), (Auburn, 4),
                (Copper, 3), (Red, 2), (Black, 3),
            ],
        },
        EthnicGroup::SubSaharanAfrican => &[
            (Black, 45), (JetBlack, 30), (VeryDarkBrown, 20), (DarkBrown, 5),
        ],
        EthnicGroup::NorthAfrican => &[
            (Black, 20), (JetBlack, 10), (VeryDarkBrown, 25),
            (DarkBrown, 20), (ChestnutBrown, 12), (DarkAuburn, 8),
            (MediumBrown, 5),
        ],
        EthnicGroup::LatinAmerican => match ethnicity {
            Ethnicity::Argentine => &[
                (Black, 5), (VeryDarkBrown, 10), (DarkBrown, 20),
                (ChestnutBrown, 15), (MediumBrown, 15), (LightBrown, 12),
                (Auburn, 5), (GoldenBlonde, 5), (HoneyBlonde, 5),
                (AshBlonde, 3), (DarkAuburn, 5),
            ],
            _ => &[
                (Black, 20), (JetBlack, 10), (VeryDarkBrown, 25),
                (DarkBrown, 20), (ChestnutBrown, 10), (MediumBrown, 8),
                (DarkAuburn, 5), (LightBrown, 2),
            ],
        },
        EthnicGroup::PacificIslander => &[
            (Black, 40), (JetBlack, 25), (VeryDarkBrown, 20),
            (DarkBrown, 10), (ChestnutBrown, 5),
        ],
        EthnicGroup::Indigenous => &[
            (Black, 40), (JetBlack, 25), (VeryDarkBrown, 20),
            (DarkBrown, 10), (ChestnutBrown, 5),
        ],
        EthnicGroup::Mixed => &[
            (Black, 15), (JetBlack, 5), (VeryDarkBrown, 12),
            (DarkBrown, 15), (ChestnutBrown, 10), (MediumBrown, 12),
            (LightBrown, 8), (Auburn, 5), (GoldenBlonde, 5),
            (HoneyBlonde, 3), (Red, 2), (Copper, 3), (AshBlonde, 3),
            (DarkAuburn, 2),
        ],
    };

    weighted_pick(rng, options)
}

/// Generate a hair texture based on ethnicity.
pub fn random_hair_texture(rng: &mut impl Rng, ethnicity: &Ethnicity) -> HairTexture {
    use HairTexture::*;

    let options: &[(HairTexture, u32)] = match ethnicity.group() {
        EthnicGroup::EastAsian => &[
            (Straight, 80), (Wavy, 18), (Curly, 2),
        ],
        EthnicGroup::SoutheastAsian => &[
            (Straight, 65), (Wavy, 30), (Curly, 5),
        ],
        EthnicGroup::SouthAsian => &[
            (Straight, 30), (Wavy, 35), (Curly, 30), (Coily, 5),
        ],
        EthnicGroup::CentralAsian => &[
            (Straight, 55), (Wavy, 35), (Curly, 10),
        ],
        EthnicGroup::MiddleEastern => &[
            (Straight, 20), (Wavy, 35), (Curly, 40), (Coily, 5),
        ],
        EthnicGroup::European => match ethnicity {
            Ethnicity::Scandinavian => &[
                (Straight, 50), (Wavy, 35), (Curly, 15),
            ],
            Ethnicity::Celtic => &[
                (Straight, 15), (Wavy, 35), (Curly, 45), (Coily, 5),
            ],
            _ => &[
                (Straight, 30), (Wavy, 40), (Curly, 25), (Coily, 5),
            ],
        },
        EthnicGroup::SubSaharanAfrican => &[
            (Coily, 40), (Kinky, 35), (Curly, 20), (Wavy, 5),
        ],
        EthnicGroup::NorthAfrican => &[
            (Straight, 15), (Wavy, 30), (Curly, 45), (Coily, 10),
        ],
        EthnicGroup::LatinAmerican => &[
            (Straight, 25), (Wavy, 35), (Curly, 30), (Coily, 10),
        ],
        EthnicGroup::PacificIslander => &[
            (Straight, 15), (Wavy, 45), (Curly, 35), (Coily, 5),
        ],
        EthnicGroup::Indigenous => match ethnicity {
            Ethnicity::Inuit => &[
                (Straight, 75), (Wavy, 20), (Curly, 5),
            ],
            _ => &[
                (Straight, 60), (Wavy, 30), (Curly, 10),
            ],
        },
        EthnicGroup::Mixed => &[
            (Straight, 20), (Wavy, 30), (Curly, 30), (Coily, 15), (Kinky, 5),
        ],
    };

    weighted_pick(rng, options)
}

/// Generate a hair length based on age.
pub fn random_hair_length(rng: &mut impl Rng, age: u8) -> HairLength {
    use HairLength::*;

    let options: &[(HairLength, u32)] = match age {
        3..=5 => &[
            (VeryShort, 5), (Short, 20), (ChinLength, 30),
            (ShoulderLength, 35), (MidBack, 10),
        ],
        6..=11 => &[
            (Short, 10), (ChinLength, 15), (ShoulderLength, 30),
            (MidBack, 30), (WaistLength, 15),
        ],
        12..=17 => &[
            (VeryShort, 3), (Short, 8), (ChinLength, 10),
            (ShoulderLength, 25), (MidBack, 30), (WaistLength, 18),
            (HipLength, 6),
        ],
        _ => &[
            (Shaved, 1), (Buzzed, 2), (VeryShort, 5), (Short, 10),
            (ChinLength, 12), (ShoulderLength, 25), (MidBack, 25),
            (WaistLength, 14), (HipLength, 6),
        ],
    };

    weighted_pick(rng, options)
}

/// Hairstyles for toddlers (3-5).
const TODDLER_STYLES: &[&str] = &[
    "loose and natural",
    "two tiny pigtails",
    "small ponytail",
    "half-up with a clip",
    "tucked behind ears",
    "with a headband",
    "two small braids",
    "messy and tousled",
    "in tiny buns",
    "with barrettes",
    "two low pigtails",
    "with a bow",
    "brushed to one side",
];

/// Hairstyles for young children (6-9).
const CHILD_STYLES: &[&str] = &[
    "loose and natural",
    "two ponytails",
    "single ponytail",
    "two braids",
    "single braid",
    "half-up half-down",
    "with a headband",
    "in a bun",
    "two buns",
    "french braid",
    "with barrettes",
    "pigtails with ribbons",
    "fishtail braid",
    "bubble ponytail",
    "braided pigtails",
    "messy bun",
    "with a scrunchie",
    "tied back with a ribbon",
    "low ponytail",
    "side ponytail",
];

/// Hairstyles for tweens (10-13).
const TWEEN_STYLES: &[&str] = &[
    "loose and natural",
    "high ponytail",
    "low ponytail",
    "messy bun",
    "two braids",
    "single braid",
    "half-up half-down",
    "french braid",
    "dutch braid",
    "fishtail braid",
    "space buns",
    "top knot",
    "side part",
    "middle part",
    "with a headband",
    "bubble ponytail",
    "braided crown",
    "waterfall braid",
    "twisted half-up",
    "claw clip bun",
    "low bun",
    "side braid",
    "two low buns",
    "slicked-back ponytail",
];

/// Hairstyles for teens (14-17).
const TEEN_STYLES: &[&str] = &[
    "loose and natural",
    "beachy waves",
    "high ponytail",
    "messy bun",
    "sleek straight",
    "half-up half-down",
    "french braid",
    "dutch braid",
    "fishtail braid",
    "space buns",
    "curtain bangs",
    "side part",
    "middle part",
    "top knot",
    "low messy bun",
    "braided crown",
    "waterfall braid",
    "bubble ponytail",
    "claw clip updo",
    "side-swept",
    "textured layers",
    "blowout",
    "twisted updo",
    "halo braid",
    "two strand twists",
    "slicked-back low bun",
    "side braid",
    "wolf cut styled",
    "butterfly clips style",
    "90s-inspired half-up",
    "loose curls",
    "straightened and sleek",
];

/// Hairstyles for adults (18+).
const ADULT_STYLES: &[&str] = &[
    "loose and natural",
    "beachy waves",
    "sleek straight",
    "voluminous blowout",
    "high ponytail",
    "low ponytail",
    "messy bun",
    "elegant updo",
    "chignon",
    "french twist",
    "half-up half-down",
    "french braid",
    "dutch braid",
    "fishtail braid",
    "braided crown",
    "waterfall braid",
    "side-swept curls",
    "retro waves",
    "pin curls",
    "curtain bangs",
    "blunt bangs",
    "wispy bangs",
    "side part",
    "middle part",
    "deep side part",
    "top knot",
    "low bun",
    "textured bob",
    "layered",
    "feathered layers",
    "tousled and textured",
    "slicked-back",
    "twisted updo",
    "halo braid",
    "goddess braids",
    "two strand twists",
    "flat twists",
    "faux hawk",
    "victory rolls",
    "barrel curls",
    "finger waves",
    "claw clip updo",
    "bubble ponytail",
    "old Hollywood waves",
    "loose romantic curls",
    "wet look",
    "boho braids",
    "milkmaid braids",
    "rope braid",
    "sock bun",
    "ballerina bun",
    "messy side braid",
    "pompadour",
];

/// Styles particularly suited for coily/kinky textures at any age.
const COILY_STYLES: &[&str] = &[
    "natural afro",
    "twist-out",
    "braid-out",
    "bantu knots",
    "cornrows",
    "box braids",
    "micro braids",
    "faux locs",
    "goddess locs",
    "Senegalese twists",
    "Havana twists",
    "puff",
    "high puff",
    "two puffs",
    "flat twists",
    "two-strand twists",
    "finger coils",
    "wash-and-go",
    "defined curls",
    "stretched afro",
    "tapered cut",
    "TWA (teeny weeny afro)",
    "frohawk",
    "braided updo",
    "halo twist",
    "marley twists",
    "passion twists",
    "crochet braids",
    "fulani braids",
    "knotless braids",
    "tribal braids",
    "feed-in braids",
    "stitch braids",
];

/// Coily/kinky styles suitable for children (3-9).
const COILY_CHILD_STYLES: &[&str] = &[
    "natural afro",
    "two puffs",
    "puff with a headband",
    "small twists",
    "cornrows",
    "beaded braids",
    "two-strand twists",
    "bantu knots",
    "braided pigtails",
    "small box braids",
    "flat twists with beads",
    "twist-out",
    "defined curls",
    "afro puffs with bows",
];

/// Generate a hairstyle appropriate for age and hair texture.
pub fn random_hair_style(rng: &mut impl Rng, age: u8, texture: &HairTexture, length: &HairLength) -> HairStyle {
    // Shaved/buzzed don't need styling
    match length {
        HairLength::Shaved => return HairStyle { name: "shaved".to_string() },
        HairLength::Buzzed => return HairStyle { name: "buzzed".to_string() },
        _ => {}
    }

    let is_coily = matches!(texture, HairTexture::Coily | HairTexture::Kinky);

    let pool: &[&str] = match (age, is_coily) {
        (3..=5, true) => COILY_CHILD_STYLES,
        (3..=5, false) => TODDLER_STYLES,
        (6..=9, true) => COILY_CHILD_STYLES,
        (6..=9, false) => CHILD_STYLES,
        (10..=13, true) => {
            // Mix child coily + some general coily
            let name = if rng.gen_bool(0.5) {
                *COILY_CHILD_STYLES.choose(rng).unwrap()
            } else {
                *COILY_STYLES.choose(rng).unwrap()
            };
            return HairStyle { name: name.to_string() };
        }
        (10..=13, false) => TWEEN_STYLES,
        (14..=17, true) => COILY_STYLES,
        (14..=17, false) => TEEN_STYLES,
        (_, true) => COILY_STYLES,
        (_, false) => ADULT_STYLES,
    };

    let name = pool.choose(rng).unwrap();
    HairStyle { name: name.to_string() }
}
