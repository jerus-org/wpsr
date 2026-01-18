use std::{fmt::Display, str::FromStr};

use clap::Parser;

#[derive(Clone, Debug, Default, Parser, PartialEq, Eq)]
pub enum Shuffle {
    #[default]
    None,
    Once,
    Twice,
}

impl Shuffle {
    pub fn new() -> Self {
        Self::default()
    }
}

impl FromStr for Shuffle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "once" => Ok(Self::Once),
            "twice" => Ok(Self::Twice),
            _ => Err(format!("Invalid shuffle strategy: {s}")),
        }
    }
}

impl Display for Shuffle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::Once => write!(f, "Once"),
            Self::Twice => write!(f, "Twice"),
        }
    }
}
