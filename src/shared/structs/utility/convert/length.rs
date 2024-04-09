#![allow(clippy::from_over_into)]

use crate::shared::structs::utility::convert::{
    ConverterType, FromStrToConverter, ParseConverterError,
};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, poise::ChoiceParameter)]
pub enum Length {
    #[name = "km"]
    Kilometer,
    #[name = "m"]
    Meter,
    #[name = "cm"]
    Centimeter,
    #[name = "inches"]
    Inch,
    #[name = "feet"]
    Foot,
    #[name = "miles"]
    Mile,
    #[name = "au"]
    Astronomical,
}

impl FromStr for Length {
    type Err = ParseConverterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.trim().to_lowercase();
        if Self::all_available_units().contains(&lowercase.as_str()) {
            Ok(Self::new(s))
        } else {
            Err(ParseConverterError)
        }
    }
}

impl FromStrToConverter for Length {
    fn from_str_to_converter(
        source: &str,
        target: &str,
        amount: f32,
    ) -> Result<ConverterType, ParseConverterError> {
        match (source.parse::<Length>(), target.parse::<Length>()) {
            (Ok(source_result), Ok(target_result)) => {
                Ok(ConverterType::Length(source_result, target_result, amount))
            }
            _ => Err(ParseConverterError),
        }
    }
}

impl Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Length as Into<String>>::into(*self))
    }
}

impl Into<&str> for Length {
    fn into(self) -> &'static str {
        match self {
            Self::Kilometer => "km",
            Self::Meter => "m",
            Self::Centimeter => "cm",
            Self::Inch => "in",
            Self::Foot => "ft",
            Self::Mile => "mi",
            Self::Astronomical => "au",
        }
    }
}

impl Into<String> for Length {
    fn into(self) -> String {
        let str: &str = self.into();
        str.to_string()
    }
}

impl Length {
    pub(self) fn new(source: &str) -> Self {
        match source {
            "km" => Self::Kilometer,
            "m" => Self::Meter,
            "cm" => Self::Centimeter,
            "in" => Self::Inch,
            "ft" => Self::Foot,
            "mi" => Self::Mile,
            "au" => Self::Astronomical,
            _ => panic!("Invalid source text."),
        }
    }

    pub fn all_available_units() -> Vec<&'static str> {
        let all_units = vec![
            Self::Kilometer,
            Self::Meter,
            Self::Centimeter,
            Self::Inch,
            Self::Foot,
            Self::Mile,
            Self::Astronomical,
        ];
        all_units.into_iter().map(Into::<&str>::into).collect()
    }
}
