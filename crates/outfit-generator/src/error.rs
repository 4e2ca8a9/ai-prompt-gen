use std::path::PathBuf;

use crate::item::Category;

/// Errors that can occur when loading items or generating outfits.
#[derive(Debug)]
pub enum Error {
    /// An I/O error while reading a file or directory.
    Io(PathBuf, std::io::Error),
    /// A TOML parsing error.
    Parse(PathBuf, toml::de::Error),
    /// A required clothing category had no compatible items available.
    NothingAvailable(Category),
    /// The wardrobe is empty — load some items first.
    EmptyWardrobe,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(path, err) => write!(f, "{}: {err}", path.display()),
            Error::Parse(path, err) => write!(f, "parse error in {}: {err}", path.display()),
            Error::NothingAvailable(cat) => {
                write!(f, "no compatible items available for {cat:?}")
            }
            Error::EmptyWardrobe => write!(f, "wardrobe is empty, load items first"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(_, e) => Some(e),
            Error::Parse(_, e) => Some(e),
            _ => None,
        }
    }
}
