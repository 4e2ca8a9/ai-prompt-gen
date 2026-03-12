use rand::Rng;
use std::fmt;

use crate::ethnicity::{EthnicGroup, Ethnicity};

/// Skin tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkinTone {
    Porcelain,
    Ivory,
    Fair,
    Light,
    LightMedium,
    Medium,
    MediumTan,
    Olive,
    Tan,
    Caramel,
    HoneyBrown,
    Golden,
    WarmBrown,
    ChestnutBrown,
    DeepBrown,
    Mahogany,
    Espresso,
    DarkEspresso,
    Ebony,
}

impl fmt::Display for SkinTone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SkinTone::Porcelain => "porcelain",
            SkinTone::Ivory => "ivory",
            SkinTone::Fair => "fair",
            SkinTone::Light => "light",
            SkinTone::LightMedium => "light-medium",
            SkinTone::Medium => "medium",
            SkinTone::MediumTan => "medium-tan",
            SkinTone::Olive => "olive",
            SkinTone::Tan => "tan",
            SkinTone::Caramel => "caramel",
            SkinTone::HoneyBrown => "honey-brown",
            SkinTone::Golden => "golden",
            SkinTone::WarmBrown => "warm brown",
            SkinTone::ChestnutBrown => "chestnut brown",
            SkinTone::DeepBrown => "deep brown",
            SkinTone::Mahogany => "mahogany",
            SkinTone::Espresso => "espresso",
            SkinTone::DarkEspresso => "dark espresso",
            SkinTone::Ebony => "ebony",
        };
        write!(f, "{s}")
    }
}

/// Eye color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EyeColor {
    DarkBrown,
    Brown,
    LightBrown,
    Amber,
    Hazel,
    Green,
    BlueGreen,
    Blue,
    LightBlue,
    IceBlue,
    Grey,
    GreyBlue,
    GreyGreen,
    Black,
    DarkAmber,
    Honey,
}

impl fmt::Display for EyeColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            EyeColor::DarkBrown => "dark brown",
            EyeColor::Brown => "brown",
            EyeColor::LightBrown => "light brown",
            EyeColor::Amber => "amber",
            EyeColor::Hazel => "hazel",
            EyeColor::Green => "green",
            EyeColor::BlueGreen => "blue-green",
            EyeColor::Blue => "blue",
            EyeColor::LightBlue => "light blue",
            EyeColor::IceBlue => "ice blue",
            EyeColor::Grey => "grey",
            EyeColor::GreyBlue => "grey-blue",
            EyeColor::GreyGreen => "grey-green",
            EyeColor::Black => "black",
            EyeColor::DarkAmber => "dark amber",
            EyeColor::Honey => "honey",
        };
        write!(f, "{s}")
    }
}

/// Pick a weighted random item. `options` is a slice of (item, weight).
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

/// Generate a skin tone appropriate for the given ethnicity.
pub fn random_skin_tone(rng: &mut impl Rng, ethnicity: &Ethnicity) -> SkinTone {
    use SkinTone::*;

    let options: &[(SkinTone, u32)] = match ethnicity.group() {
        EthnicGroup::EastAsian => &[
            (Porcelain, 10), (Ivory, 20), (Fair, 20), (Light, 20),
            (LightMedium, 15), (Medium, 10), (Golden, 5),
        ],
        EthnicGroup::SoutheastAsian => &[
            (Light, 5), (LightMedium, 10), (Medium, 20), (MediumTan, 20),
            (Tan, 20), (Golden, 15), (HoneyBrown, 10),
        ],
        EthnicGroup::SouthAsian => &[
            (Light, 5), (LightMedium, 8), (Medium, 12), (MediumTan, 15),
            (Olive, 5), (Tan, 10), (Caramel, 10), (HoneyBrown, 10),
            (WarmBrown, 10), (ChestnutBrown, 10), (DeepBrown, 5),
        ],
        EthnicGroup::CentralAsian => &[
            (Fair, 10), (Light, 15), (LightMedium, 20), (Medium, 25),
            (MediumTan, 15), (Olive, 10), (Tan, 5),
        ],
        EthnicGroup::MiddleEastern => &[
            (Fair, 5), (Light, 10), (LightMedium, 15), (Medium, 15),
            (Olive, 20), (MediumTan, 15), (Tan, 10), (HoneyBrown, 10),
        ],
        EthnicGroup::European => match ethnicity {
            Ethnicity::Scandinavian | Ethnicity::Celtic => &[
                (Porcelain, 20), (Ivory, 25), (Fair, 25), (Light, 20),
                (LightMedium, 10),
            ],
            Ethnicity::SouthernEuropean => &[
                (Fair, 5), (Light, 15), (LightMedium, 20), (Medium, 20),
                (Olive, 25), (MediumTan, 15),
            ],
            _ => &[
                (Porcelain, 8), (Ivory, 15), (Fair, 20), (Light, 25),
                (LightMedium, 18), (Medium, 10), (Olive, 4),
            ],
        },
        EthnicGroup::SubSaharanAfrican => match ethnicity {
            Ethnicity::EastAfrican => &[
                (WarmBrown, 10), (ChestnutBrown, 15), (DeepBrown, 25),
                (Mahogany, 25), (Espresso, 20), (DarkEspresso, 5),
            ],
            _ => &[
                (ChestnutBrown, 5), (DeepBrown, 10), (Mahogany, 20),
                (Espresso, 25), (DarkEspresso, 25), (Ebony, 15),
            ],
        },
        EthnicGroup::NorthAfrican => &[
            (LightMedium, 10), (Medium, 15), (Olive, 20), (MediumTan, 20),
            (Tan, 15), (Caramel, 10), (HoneyBrown, 10),
        ],
        EthnicGroup::LatinAmerican => match ethnicity {
            Ethnicity::Argentine => &[
                (Fair, 10), (Light, 20), (LightMedium, 25), (Medium, 20),
                (Olive, 15), (MediumTan, 10),
            ],
            _ => &[
                (Light, 5), (LightMedium, 10), (Medium, 15), (Olive, 10),
                (MediumTan, 15), (Tan, 15), (Caramel, 10),
                (HoneyBrown, 10), (WarmBrown, 10),
            ],
        },
        EthnicGroup::PacificIslander => match ethnicity {
            Ethnicity::Aboriginal => &[
                (ChestnutBrown, 10), (DeepBrown, 25), (Mahogany, 30),
                (Espresso, 25), (DarkEspresso, 10),
            ],
            _ => &[
                (MediumTan, 10), (Tan, 20), (Caramel, 20),
                (HoneyBrown, 20), (WarmBrown, 20), (ChestnutBrown, 10),
            ],
        },
        EthnicGroup::Indigenous => match ethnicity {
            Ethnicity::Inuit => &[
                (LightMedium, 10), (Medium, 25), (MediumTan, 30),
                (Tan, 20), (Golden, 15),
            ],
            _ => &[
                (Medium, 10), (MediumTan, 15), (Tan, 20), (Caramel, 20),
                (HoneyBrown, 20), (WarmBrown, 15),
            ],
        },
        EthnicGroup::Mixed => &[
            (Fair, 5), (Light, 8), (LightMedium, 10), (Medium, 12),
            (MediumTan, 10), (Olive, 8), (Tan, 10), (Caramel, 10),
            (HoneyBrown, 10), (WarmBrown, 7), (ChestnutBrown, 5),
            (DeepBrown, 5),
        ],
    };

    weighted_pick(rng, options)
}

/// Generate an eye color appropriate for the given ethnicity.
pub fn random_eye_color(rng: &mut impl Rng, ethnicity: &Ethnicity) -> EyeColor {
    use EyeColor::*;

    let options: &[(EyeColor, u32)] = match ethnicity.group() {
        EthnicGroup::EastAsian => &[
            (DarkBrown, 50), (Brown, 30), (Black, 15), (LightBrown, 5),
        ],
        EthnicGroup::SoutheastAsian => &[
            (DarkBrown, 45), (Brown, 35), (Black, 15), (LightBrown, 5),
        ],
        EthnicGroup::SouthAsian => &[
            (DarkBrown, 40), (Brown, 30), (Black, 15), (LightBrown, 8),
            (Amber, 4), (Hazel, 2), (Green, 1),
        ],
        EthnicGroup::CentralAsian => &[
            (DarkBrown, 30), (Brown, 30), (LightBrown, 15),
            (Hazel, 10), (Green, 8), (Grey, 5), (Blue, 2),
        ],
        EthnicGroup::MiddleEastern => &[
            (DarkBrown, 30), (Brown, 25), (LightBrown, 10),
            (Amber, 8), (Hazel, 10), (Green, 10), (Honey, 5), (Blue, 2),
        ],
        EthnicGroup::European => match ethnicity {
            Ethnicity::Scandinavian => &[
                (Blue, 30), (LightBlue, 15), (IceBlue, 10), (GreyBlue, 10),
                (Grey, 10), (Green, 10), (Hazel, 8), (Brown, 5), (GreyGreen, 2),
            ],
            Ethnicity::Celtic => &[
                (Blue, 20), (LightBlue, 10), (Green, 20), (GreyGreen, 10),
                (Grey, 10), (Hazel, 15), (Brown, 10), (GreyBlue, 5),
            ],
            Ethnicity::SouthernEuropean => &[
                (DarkBrown, 15), (Brown, 25), (LightBrown, 10),
                (Hazel, 15), (Green, 15), (Amber, 5), (Blue, 10), (Grey, 5),
            ],
            _ => &[
                (Blue, 20), (LightBlue, 8), (Green, 15), (GreyGreen, 5),
                (Grey, 8), (GreyBlue, 5), (Hazel, 15), (Brown, 15),
                (LightBrown, 5), (Amber, 4),
            ],
        },
        EthnicGroup::SubSaharanAfrican => &[
            (DarkBrown, 55), (Brown, 30), (Black, 10), (DarkAmber, 5),
        ],
        EthnicGroup::NorthAfrican => &[
            (DarkBrown, 25), (Brown, 25), (LightBrown, 10),
            (Hazel, 15), (Green, 10), (Amber, 8), (Honey, 5), (Grey, 2),
        ],
        EthnicGroup::LatinAmerican => &[
            (DarkBrown, 25), (Brown, 30), (LightBrown, 10),
            (Hazel, 12), (Green, 8), (Amber, 5), (Honey, 5), (Blue, 3), (Grey, 2),
        ],
        EthnicGroup::PacificIslander => &[
            (DarkBrown, 50), (Brown, 35), (Black, 10), (LightBrown, 5),
        ],
        EthnicGroup::Indigenous => &[
            (DarkBrown, 45), (Brown, 35), (Black, 15), (LightBrown, 5),
        ],
        EthnicGroup::Mixed => &[
            (DarkBrown, 15), (Brown, 20), (LightBrown, 10),
            (Amber, 8), (Hazel, 15), (Green, 10), (Blue, 8),
            (Grey, 5), (Honey, 5), (GreyGreen, 4),
        ],
    };

    weighted_pick(rng, options)
}
