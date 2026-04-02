pub mod filenames;
pub use filenames::SlugifyFilenames;
pub mod string;
pub use string::SlugifyString;

pub mod parameters;
pub use parameters::{argv_fallback_to_stdin_lines, SlugifyParameters, PREFER_DECODED_DEFAULT};

pub mod aliasing;
pub use aliasing::heck_aliases;

pub mod verbosity;
pub use verbosity::Verbosity;
