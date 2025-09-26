use clap::builder::PossibleValue;
use clap::ValueEnum;

use crate::heck_aliases;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Ord, PartialOrd, Eq, PartialEq, Default)]
pub enum Verbosity {
    None,
    Quiet,
    Warning,
    #[default]
    Info,
    Hint,
    Debug,
}
impl Display for Verbosity {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let names = self.variant_names();
        let variant = names[0];

        write!(f, "{variant}")
    }
}

impl From<u8> for Verbosity {
    fn from(level: u8) -> Verbosity {
        match level {
            0 => Verbosity::None,
            1 => Verbosity::Quiet,
            2 => Verbosity::Warning,
            3 => Verbosity::Info,
            4 => Verbosity::Hint,
            _ => Verbosity::Debug,
        }
    }
}
impl Verbosity {
    #[rustfmt::skip]
    pub fn level(&self) -> u8 {
        match self {
            Verbosity::None =>    0,
            Verbosity::Quiet =>   1,
            Verbosity::Warning => 2,
            Verbosity::Info =>    3,
            Verbosity::Hint =>    4,
            Verbosity::Debug =>   5,
        }
    }

    #[rustfmt::skip]
    pub fn variant_names(&self) -> [&'static str; 3] {
        match self {
            Verbosity::None =>    ["none",    "null",   "0"],
            Verbosity::Quiet =>   ["quiet",   "silent", "1"],
            Verbosity::Warning => ["warning", "warn",   "2"],
            Verbosity::Info =>    ["info",    "extra",  "3"],
            Verbosity::Hint =>    ["hint",    "tip",    "4"],
            Verbosity::Debug =>   ["debug",   "dbg",    "5"],
        }
    }
    pub fn aliases(&self) -> Vec<String> {
        let mut aliases = Vec::<String>::new();
        for alias in self
            .variant_names()
            .iter()
            .map(|variant| heck_aliases(variant))
            .flatten()
        {
            if !aliases.contains(&alias) {
                aliases.push(alias)
            }
        }
        aliases
    }
    pub fn variants<'a>() -> &'a [Verbosity] {
        &[
            Verbosity::None,
            Verbosity::Quiet,
            Verbosity::Warning,
            Verbosity::Info,
            Verbosity::Hint,
            Verbosity::Debug,
        ]
    }
}
impl ValueEnum for Verbosity {
    fn value_variants<'a>() -> &'a [Verbosity] {
        Verbosity::variants()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        let mut pv = PossibleValue::new(self.to_string());
        for alias in self.aliases() {
            pv = pv.alias(alias);
        }
        Some(pv)
    }

    fn from_str(val: &str, ignore_case: bool) -> std::result::Result<Verbosity, String> {
        let val = if ignore_case {
            val.trim().to_lowercase().to_string()
        } else {
            val.trim().to_string()
        };
        for variant in Verbosity::variants() {
            if variant.aliases().into_iter().any(|alias| {
                let alias = if ignore_case {
                    alias.to_lowercase().to_string()
                } else {
                    alias.to_string()
                };
                alias == val
            }) {
                return Ok(variant.clone());
            }
        }
        return Err(val.to_string());
    }
}
