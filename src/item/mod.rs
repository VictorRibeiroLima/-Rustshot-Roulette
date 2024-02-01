use std::{collections::HashMap, fmt::Display};

pub mod list;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
pub enum Item {
    Beer = 0,
    Saw = 1,
    MagnifyingGlass = 2,
    Cigarette = 3,
    Handcuffs = 4,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let printable = match self {
            Item::Beer => "Beer",
            Item::Saw => "Saw",
            Item::MagnifyingGlass => "Magnifying Glass",
            Item::Cigarette => "Cigarette",
            Item::Handcuffs => "Handcuffs",
        };
        write!(f, "{}", printable)
    }
}

impl From<u8> for Item {
    fn from(index: u8) -> Self {
        match index {
            0 => Item::Beer,
            1 => Item::Saw,
            2 => Item::MagnifyingGlass,
            3 => Item::Cigarette,
            4 => Item::Handcuffs,
            _ => Item::Beer,
        }
    }
}

impl Item {
    pub fn get_name(index: u8) -> String {
        match index {
            0 => "Beer".to_string(),
            1 => "Saw".to_string(),
            2 => "Magnifying Glass".to_string(),
            3 => "Cigarette".to_string(),
            4 => "Handcuffs".to_string(),
            _ => "Unknown".to_string(),
        }
    }
}
