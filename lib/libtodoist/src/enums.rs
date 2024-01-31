use core::result::Result;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use core::fmt;

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
pub enum ViewStyle { List, Board, Calendar }

impl fmt::Display for ViewStyle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ViewStyle::List => write!(f, "list"),
            ViewStyle::Board => write!(f, "board"),
            ViewStyle::Calendar => write!(f, "calendar"),
        }
    }
}

impl ViewStyle {
    pub fn variants() -> Vec<&'static str> {
        vec!["list", "board", "calendar"]
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

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::BerryRed => write!(f, "berry_red"),
            Color::Blue => write!(f, "blue"),
            Color::Charcoal => write!(f, "charcoal"),
            Color::Grape => write!(f, "grape"),
            Color::Green => write!(f, "green"),
            Color::Grey => write!(f, "grey"),
            Color::Lavender => write!(f, "lavender"),
            Color::LightBlue => write!(f, "light_blue"),
            Color::LimeGreen => write!(f, "lime_green"),
            Color::Magenta => write!(f, "magenta"),
            Color::MintGreen => write!(f, "mint_green"),
            Color::OliveGreen => write!(f, "olive_green"),
            Color::Orange => write!(f, "orange"),
            Color::Red => write!(f, "red"),
            Color::Salmon => write!(f, "salmon"),
            Color::SkyBlue => write!(f, "sky_blue"),
            Color::Taupe => write!(f, "taupe"),
            Color::Teal => write!(f, "teal"),
            Color::Violet => write!(f, "violet"),
            Color::Yellow => write!(f, "yellow"),
        }
    }
}

impl Color {
    pub fn variants() -> Vec<&'static str> {
        vec![
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
        vec!["low", "normal", "high", "urgent"]
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnit { Minute, Hour }

imp_from_str!(ViewStyle);
imp_from_str!(Color);
imp_from_str!(Priority);
