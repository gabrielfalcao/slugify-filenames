pub mod filenames;
pub use filenames::SlugifyFilenames;
pub mod string;
pub use string::SlugifyString;
pub mod parameters;
pub use parameters::SlugifyParameters;

pub(crate) mod aliasing;
pub(crate) use aliasing::heck_aliases;

pub mod verbosity;
pub use verbosity::Verbosity;
