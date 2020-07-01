use std::num::ParseIntError;

use crossterm::style::Color;
use serde::{de, Deserialize};

#[derive(Debug)]
pub struct SColor(Color);

const VALID_COLOR_ERRMSG: &str = "Is not a valid color with 7 hex digits 0-9a-fA-F";

impl<'de> Deserialize<'de> for SColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        use serde::de::Error;

        // Deserialize the string and get individual components
        let color = String::deserialize(deserializer)?;

        if !color.starts_with('#') {
            D::Error::custom("Does not start with '#'");
        }

        if color.len() != 7 {
            D::Error::custom(VALID_COLOR_ERRMSG);
        }

        color_from_str(&color[1..]).map_err(|_| D::Error::custom(VALID_COLOR_ERRMSG))
    }
}

fn color_from_str(hex: &str) -> Result<SColor, ParseIntError> {
    let r = u8::from_str_radix(&hex[0..2], 16)?;
    let g = u8::from_str_radix(&hex[2..4], 16)?;
    let b = u8::from_str_radix(&hex[4..6], 16)?;

    Ok(SColor(Color::Rgb { r, g, b }))
}

impl AsRef<Color> for SColor {
    fn as_ref(&self) -> &Color {
        &self.0
    }
}
