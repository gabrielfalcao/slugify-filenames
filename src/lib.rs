pub mod cli;
pub use cli::{heck_aliases, SlugifyFilenames, SlugifyParameters, SlugifyString};
pub mod errors;
pub use errors::{Error, Result};
pub mod string;
pub use string::{slugify_string, STRING_PATTERN, DEFAULT_SEPARATOR};
