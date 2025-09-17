use crate::string::*;
use crate::errors::*;
use clap::Args;
use sanitation::SString;
#[derive(Args, Debug, Clone)]
#[group()]
pub struct SlugifyParameters {
    #[arg(short, long, default_value = "-")]
    separator: Option<char>,

    #[arg(short, long)]
    no_trim_matches: bool,

    #[arg(short = 'L', long)]
    no_lowercase: bool,
}

impl SlugifyParameters {
    pub fn slugify_string(&self, string: impl std::fmt::Display) -> Result<String> {
        let string = SString::new(&strip_ansi_escapes::strip(&string.to_string())).unchecked_safe();
        Ok(crate::string::slugify_string(
            string,
            self.non_option_separator(),
            self.trim_matches(),
            self.lowercase()
        )?)
    }
    pub fn separator(&self) -> Option<char> {
        match self.separator {
            Some(separator) => Some(separator),
            None => None,
        }
    }
    pub fn non_option_separator(&self) -> char {
        match self.separator {
            Some(separator) => separator,
            None => DEFAULT_SEPARATOR,
        }
    }
    pub fn trim_matches(&self) -> bool {
        !self.no_trim_matches
    }
    pub fn lowercase(&self) -> bool {
        !self.no_lowercase
    }
}
