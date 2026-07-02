use crate::errors::*;
use crate::string::*;
use clap::Args;

#[derive(Args, Debug, Clone)]
#[group()]
pub struct SlugifyParameters {
    #[arg(short, long, default_value = "-")]
    separator: Option<char>,

    #[arg(
        short,
        long,
        help = "lowercase slugified filenames. The default is to not change the case so that, for example, a file named \"README.md\" does not become \"readme.md\""
    )]
    lowercase: bool,
}

impl SlugifyParameters {
    pub fn slugify_string(&self, string: impl std::fmt::Display) -> Result<String> {
        Ok(crate::string::slugify_string(string)?)
    }
    pub fn separator(&self) -> Option<char> {
        match self.separator {
            Some(separator) => Some(separator),
            None => None,
        }
    }
    pub fn lowercase(&self) -> bool {
        self.lowercase
    }
    pub fn non_option_separator(&self) -> char {
        match self.separator {
            Some(separator) => separator,
            None => DEFAULT_SEPARATOR,
        }
    }
}
