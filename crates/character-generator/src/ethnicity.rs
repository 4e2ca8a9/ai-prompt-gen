use rand::Rng;
use std::fmt;

/// The broad ethnic background of a character.
/// Determines realistic distributions for skin tone, eye color, and hair color.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ethnicity {
    // East Asian
    Chinese,
    Japanese,
    Korean,
    Vietnamese,
    Thai,
    Filipino,

    // Southeast Asian
    Indonesian,
    Malaysian,
    Cambodian,

    // South Asian
    Indian,
    Pakistani,
    Bangladeshi,
    SriLankan,

    // Central Asian
    Kazakh,
    Uzbek,

    // Middle Eastern / West Asian
    Arab,
    Persian,
    Turkish,
    Kurdish,

    // European
    NorthernEuropean,
    WesternEuropean,
    EasternEuropean,
    SouthernEuropean,
    Scandinavian,
    Celtic,

    // African
    WestAfrican,
    EastAfrican,
    SouthernAfrican,
    NorthAfrican,
    CentralAfrican,

    // Latin American
    Mexican,
    Brazilian,
    Colombian,
    Argentine,
    Peruvian,
    Caribbean,

    // Pacific Islander / Oceanian
    Hawaiian,
    Samoan,
    Maori,
    Aboriginal,

    // North American Indigenous
    NativeAmerican,
    Inuit,

    // Mixed
    Mixed,
}

impl Ethnicity {
    /// All variants for random selection.
    pub const ALL: &[Ethnicity] = &[
        Ethnicity::Chinese,
        Ethnicity::Japanese,
        Ethnicity::Korean,
        Ethnicity::Vietnamese,
        Ethnicity::Thai,
        Ethnicity::Filipino,
        Ethnicity::Indonesian,
        Ethnicity::Malaysian,
        Ethnicity::Cambodian,
        Ethnicity::Indian,
        Ethnicity::Pakistani,
        Ethnicity::Bangladeshi,
        Ethnicity::SriLankan,
        Ethnicity::Kazakh,
        Ethnicity::Uzbek,
        Ethnicity::Arab,
        Ethnicity::Persian,
        Ethnicity::Turkish,
        Ethnicity::Kurdish,
        Ethnicity::NorthernEuropean,
        Ethnicity::WesternEuropean,
        Ethnicity::EasternEuropean,
        Ethnicity::SouthernEuropean,
        Ethnicity::Scandinavian,
        Ethnicity::Celtic,
        Ethnicity::WestAfrican,
        Ethnicity::EastAfrican,
        Ethnicity::SouthernAfrican,
        Ethnicity::NorthAfrican,
        Ethnicity::CentralAfrican,
        Ethnicity::Mexican,
        Ethnicity::Brazilian,
        Ethnicity::Colombian,
        Ethnicity::Argentine,
        Ethnicity::Peruvian,
        Ethnicity::Caribbean,
        Ethnicity::Hawaiian,
        Ethnicity::Samoan,
        Ethnicity::Maori,
        Ethnicity::Aboriginal,
        Ethnicity::NativeAmerican,
        Ethnicity::Inuit,
        Ethnicity::Mixed,
    ];

    /// Pick a random ethnicity.
    pub fn random(rng: &mut impl Rng) -> Self {
        let idx = rng.gen_range(0..Self::ALL.len());
        Self::ALL[idx]
    }

    /// Broad geographic grouping, used for shared trait distributions.
    pub fn group(&self) -> EthnicGroup {
        match self {
            Ethnicity::Chinese
            | Ethnicity::Japanese
            | Ethnicity::Korean
            | Ethnicity::Vietnamese
            | Ethnicity::Thai
            | Ethnicity::Filipino => EthnicGroup::EastAsian,

            Ethnicity::Indonesian
            | Ethnicity::Malaysian
            | Ethnicity::Cambodian => EthnicGroup::SoutheastAsian,

            Ethnicity::Indian
            | Ethnicity::Pakistani
            | Ethnicity::Bangladeshi
            | Ethnicity::SriLankan => EthnicGroup::SouthAsian,

            Ethnicity::Kazakh
            | Ethnicity::Uzbek => EthnicGroup::CentralAsian,

            Ethnicity::Arab
            | Ethnicity::Persian
            | Ethnicity::Turkish
            | Ethnicity::Kurdish => EthnicGroup::MiddleEastern,

            Ethnicity::NorthernEuropean
            | Ethnicity::WesternEuropean
            | Ethnicity::EasternEuropean
            | Ethnicity::SouthernEuropean
            | Ethnicity::Scandinavian
            | Ethnicity::Celtic => EthnicGroup::European,

            Ethnicity::WestAfrican
            | Ethnicity::EastAfrican
            | Ethnicity::SouthernAfrican
            | Ethnicity::CentralAfrican => EthnicGroup::SubSaharanAfrican,

            Ethnicity::NorthAfrican => EthnicGroup::NorthAfrican,

            Ethnicity::Mexican
            | Ethnicity::Brazilian
            | Ethnicity::Colombian
            | Ethnicity::Argentine
            | Ethnicity::Peruvian
            | Ethnicity::Caribbean => EthnicGroup::LatinAmerican,

            Ethnicity::Hawaiian
            | Ethnicity::Samoan
            | Ethnicity::Maori
            | Ethnicity::Aboriginal => EthnicGroup::PacificIslander,

            Ethnicity::NativeAmerican
            | Ethnicity::Inuit => EthnicGroup::Indigenous,

            Ethnicity::Mixed => EthnicGroup::Mixed,
        }
    }
}

impl fmt::Display for Ethnicity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ethnicity::Chinese => "Chinese",
            Ethnicity::Japanese => "Japanese",
            Ethnicity::Korean => "Korean",
            Ethnicity::Vietnamese => "Vietnamese",
            Ethnicity::Thai => "Thai",
            Ethnicity::Filipino => "Filipino",
            Ethnicity::Indonesian => "Indonesian",
            Ethnicity::Malaysian => "Malaysian",
            Ethnicity::Cambodian => "Cambodian",
            Ethnicity::Indian => "Indian",
            Ethnicity::Pakistani => "Pakistani",
            Ethnicity::Bangladeshi => "Bangladeshi",
            Ethnicity::SriLankan => "Sri Lankan",
            Ethnicity::Kazakh => "Kazakh",
            Ethnicity::Uzbek => "Uzbek",
            Ethnicity::Arab => "Arab",
            Ethnicity::Persian => "Persian",
            Ethnicity::Turkish => "Turkish",
            Ethnicity::Kurdish => "Kurdish",
            Ethnicity::NorthernEuropean => "Northern European",
            Ethnicity::WesternEuropean => "Western European",
            Ethnicity::EasternEuropean => "Eastern European",
            Ethnicity::SouthernEuropean => "Southern European",
            Ethnicity::Scandinavian => "Scandinavian",
            Ethnicity::Celtic => "Celtic",
            Ethnicity::WestAfrican => "West African",
            Ethnicity::EastAfrican => "East African",
            Ethnicity::SouthernAfrican => "Southern African",
            Ethnicity::NorthAfrican => "North African",
            Ethnicity::CentralAfrican => "Central African",
            Ethnicity::Mexican => "Mexican",
            Ethnicity::Brazilian => "Brazilian",
            Ethnicity::Colombian => "Colombian",
            Ethnicity::Argentine => "Argentine",
            Ethnicity::Peruvian => "Peruvian",
            Ethnicity::Caribbean => "Caribbean",
            Ethnicity::Hawaiian => "Hawaiian",
            Ethnicity::Samoan => "Samoan",
            Ethnicity::Maori => "Māori",
            Ethnicity::Aboriginal => "Aboriginal Australian",
            Ethnicity::NativeAmerican => "Native American",
            Ethnicity::Inuit => "Inuit",
            Ethnicity::Mixed => "Mixed",
        };
        write!(f, "{s}")
    }
}

/// Broad grouping for shared trait distributions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EthnicGroup {
    EastAsian,
    SoutheastAsian,
    SouthAsian,
    CentralAsian,
    MiddleEastern,
    European,
    SubSaharanAfrican,
    NorthAfrican,
    LatinAmerican,
    PacificIslander,
    Indigenous,
    Mixed,
}
