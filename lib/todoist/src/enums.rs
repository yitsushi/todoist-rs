use core::result::Result;
use serde::{Serialize, Serializer};

macro_rules! impl_serialize_with_to_string {
    ($name:ident) => {
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                serializer.serialize_str(&self.to_string())
            }
        }
    };
}

macro_rules! imp_from_str {
    ($name:ident) => {
        impl ::std::str::FromStr for $name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if let Some(value) = Self::from_str(s) {
                    Ok(value)
                } else {
                    Err("invalid value".to_string())
                }
            }
        }
    };
}

#[derive(Debug,Clone)]
pub enum ViewStyle { List, Board }

impl ViewStyle {
    pub fn to_string(&self) -> String {
        String::from(match self {
            ViewStyle::List => "list",
            ViewStyle::Board => "board",
        })
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "list" => Some(ViewStyle::List),
            "board" => Some(ViewStyle::Board),
            _      => None,
        }
    }
}

#[derive(Debug,Clone)]
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

impl_serialize_with_to_string!(ViewStyle);
impl_serialize_with_to_string!(Color);

imp_from_str!(ViewStyle);
imp_from_str!(Color);