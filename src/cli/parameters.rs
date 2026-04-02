use crate::errors::Result;
use crate::string::{get_stdin_text, DEFAULT_SEPARATOR};

use clap::Args;
use iocore::Path;
use std::convert::AsRef;

pub const PREFER_DECODED_DEFAULT: bool = true;

#[derive(Args, Debug, Clone, Copy)]
#[group()]
pub struct SlugifyParameters {
    #[arg(short, long, default_value = "-")]
    separator: Option<char>,

    #[arg(short, long)]
    pub no_join: bool,
}

impl SlugifyParameters {
    pub fn slugify_string(&self, string: impl std::fmt::Display) -> Result<String> {
        Ok(crate::string::slugify_string(
            string,
            self.non_option_separator(),
        )?)
    }
    pub fn slugify_path<T: AsRef<std::path::Path>>(&self, path_ref: T) -> Result<Path> {
        Ok(crate::string::slugify_path(
            path_ref,
            self.non_option_separator(),
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
}

pub fn argv_fallback_to_stdin_lines(val: &str) -> ::std::result::Result<String, String> {
    let text = if val.is_empty() {
        match get_stdin_text() {
            Ok(lines) => lines,
            Err(error) => {
                return Err(format!("missing TEXT via command-line argument(s) or stdin"))
            }
        }
    } else {
        val.to_string()
    };
    Ok(text)
}
