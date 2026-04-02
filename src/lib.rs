pub mod cli;
pub use cli::{
    argv_fallback_to_stdin_lines, heck_aliases, SlugifyFilenames, SlugifyParameters, SlugifyString,
    Verbosity, PREFER_DECODED_DEFAULT,
};

pub mod errors;
pub use errors::{Error, Result};
pub mod string;
pub use string::{
    get_stdin_lines, get_stdin_text, regex_pattern, slugify_string, string_pattern,
    DEFAULT_SEPARATOR,
};
