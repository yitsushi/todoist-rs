use core::result::Result;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

macro_rules! imp_from_str {
    ($name:ident) => {
        impl ::std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some(value) = Self::from_str(s) {
                    Ok(value)
                } else {
                    Err(format!("valid options: {}", Self::variants().join(", ")))
                }
            }
        }
    };
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ViewStyle { List, Board }

impl ViewStyle {
    pub fn to_string(&self) -> String {
        String::from(match self {
            ViewStyle::List => "list",
            ViewStyle::Board => "board",
        })
    }

    pub fn variants() -> Vec<&'static str> {
        return vec!["list", "board"]
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "list" => Some(ViewStyle::List),
            "board" => Some(ViewStyle::Board),
            _      => None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    BerryRed, Blue,
    Charcoal,
    Grape, Green, Grey,
    Lavender, LightBlue, LimeGreen,
    Magenta, MintGreen,
    OliveGreen, Orange,
    Red,
    Salmon, SkyBlue,
    Taupe, Teal,
    Violet,
    Yellow,
}

impl Color {
    pub fn to_string(&self) -> String {
        String::from(match self {
            Color::BerryRed => "berry_red",
            Color::Blue => "blue",
            Color::Charcoal => "charcoal",
            Color::Grape => "grape",
            Color::Green => "green",
            Color::Grey => "grey",
            Color::Lavender => "lavender",
            Color::LightBlue => "light_blue",
            Color::LimeGreen => "lime_green",
            Color::Magenta => "magenta",
            Color::MintGreen => "mint_green",
            Color::OliveGreen => "olive_green",
            Color::Orange => "orange",
            Color::Red => "red",
            Color::Salmon => "salmon",
            Color::SkyBlue => "sky_blue",
            Color::Taupe => "taupe",
            Color::Teal => "teal",
            Color::Violet => "violet",
            Color::Yellow => "yellow",
        })
    }

    pub fn variants() -> Vec<&'static str> {
        return vec![
            "berry_red",
            "blue",
            "charcoal",
            "grape",
            "green",
            "grey",
            "lavender",
            "light_blue",
            "lime_green",
            "magenta",
            "mint_green",
            "olive_green",
            "orange",
            "red",
            "salmon",
            "sky_blue",
            "taupe",
            "teal",
            "violet",
            "yellow",
        ]
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "berry_red" => Some(Color::BerryRed),
            "blue" => Some(Color::Blue),
            "charcoal" => Some(Color::Charcoal),
            "grape" => Some(Color::Grape),
            "green" => Some(Color::Green),
            "grey" => Some(Color::Grey),
            "lavender" => Some(Color::Lavender),
            "light_blue" => Some(Color::LightBlue),
            "lime_green" => Some(Color::LimeGreen),
            "magenta" => Some(Color::Magenta),
            "mint_green" => Some(Color::MintGreen),
            "olive_green" => Some(Color::OliveGreen),
            "orange" => Some(Color::Orange),
            "red" => Some(Color::Red),
            "salmon" => Some(Color::Salmon),
            "sky_blue" => Some(Color::SkyBlue),
            "taupe" => Some(Color::Taupe),
            "teal" => Some(Color::Teal),
            "violet" => Some(Color::Violet),
            "yellow" => Some(Color::Yellow),
            _      => None,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
pub enum Priority {
    Low = 1,
    Normal = 2,
    High = 3,
    Urgent = 4,
}

impl Priority {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Priority::Low),
            "normal" => Some(Priority::Normal),
            "high" => Some(Priority::High),
            "urgent" => Some(Priority::Urgent),
            _      => None,
        }
    }

    pub fn variants() -> Vec<&'static str> {
        return vec!["low", "normal", "high", "urgent"]
    }
}

imp_from_str!(ViewStyle);
imp_from_str!(Color);
imp_from_str!(Priority);
