use serde::{Serialize, Serializer};

pub enum ViewStyle { List, Board }

impl ViewStyle {
    pub fn to_string(&self) -> String {
        String::from(match self {
            ViewStyle::List => "list",
            ViewStyle::Board => "board",
        })
    }
}

impl Serialize for ViewStyle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

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
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}