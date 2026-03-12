pub mod error;
pub mod item;
pub mod outfit;
pub mod wardrobe;

pub use error::Error;
pub use item::{Category, Item, Slot};
pub use outfit::{generate_outfit, Outfit};
pub use wardrobe::Wardrobe;
