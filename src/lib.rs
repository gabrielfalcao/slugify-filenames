pub mod cli;
pub use cli::{heck_aliases, SlugifyFilenames, SlugifyParameters, SlugifyString};
pub mod errors;
pub use errors::{Error, Result};

pub(crate) mod string;
pub use string::{
    list_of_trimmed_strings, slugify_string, DEFAULT_SEPARATOR, SPECIAL_PATTERN_CHARS,
    STRING_REGEX, UNNEEDED_UNIQUEFY_REGEX,
};
