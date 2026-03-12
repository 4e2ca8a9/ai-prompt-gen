/// A complete outfit composed of individual clothing items.
pub struct Outfit {
    pub top: Top,
    pub bottom: Bottom,
    pub shoes: Shoes,
    pub accessories: Vec<Accessory>,
}

/// Upper-body garment.
pub enum Top {
    Blouse,
    TShirt,
    Sweater,
    CropTop,
    Tank,
}

/// Lower-body garment.
pub enum Bottom {
    Skirt,
    Jeans,
    Trousers,
    Shorts,
    Leggings,
}

/// Footwear.
pub enum Shoes {
    Sneakers,
    Heels,
    Boots,
    Sandals,
    Flats,
}

/// An accessory item.
pub enum Accessory {
    Hat,
    Scarf,
    Necklace,
    Bag,
    Sunglasses,
}

/// Generate a random outfit.
///
/// # Panics
///
/// Not yet implemented.
pub fn generate_outfit() -> Outfit {
    todo!()
}
