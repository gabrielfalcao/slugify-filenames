//!
//! # Usage Example
//!
//!
//! ```
//! use slugify_filenames::slugify_string;
//!
//! // passing `false` in the second argument to prevent downcasing the string
//! let result = slugify_string("Imagine Thís STRING, àscii Safê and Filename-sáfè", false)?;
//! assert_eq!(result, "Imagine-This-STRING-ascii-Safe-and-Filename-safe");
//!
//! // passing `true` in the second argument to downcase the string
//! let result = slugify_string("Generated_FILE_ymf1a3ymf1a3ymf1.png.0.gz", true)?;
//! assert_eq!(result, "generated_file_ymf1a3ymf1a3ymf1.png.gz");
//!
//! # Ok::<(), slugify_filenames::errors::Error>(())
//! ```
//!

#[doc(hidden)]
pub mod cli;
pub(crate) use cli::heck_aliases;
#[doc(hidden)]
pub use cli::{SlugifyFilenames, SlugifyParameters, SlugifyString};

#[doc(hidden)]
pub mod errors;
pub use errors::{Error, Result};

#[doc(hidden)]
pub(crate) mod string;
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use string::{
    list_of_trimmed_strings, DEFAULT_SEPARATOR, SPECIAL_PATTERN_CHARS,
    STRING_REGEX, UNNEEDED_UNIQUEFY_REGEX,
};
pub use string::{
    slugify_string
};
