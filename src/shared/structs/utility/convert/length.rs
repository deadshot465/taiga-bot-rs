#![allow(clippy::from_over_into)]
use crate::shared::structs::utility::convert::{
    ConverterType, FromStrToConverter, ParseConverterError,
};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Length {
    Kilometer,
    Meter,
    Centimeter,
    Inch,
    Foot,
    Mile,
    Astronomical,
}

impl FromStr for Length {
    type Err = ParseConverterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.trim().to_lowercase();
        if Self::all_available_units().contains(&lowercase.as_str()) {
            Ok(Self::new(s))
        } else {
            Err(ParseConverterError("Failed to parse length unit."))
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
            _ => Err(ParseConverterError("Invalid length unit.")),
        }
    }
}

impl ToString for Length {
    fn to_string(&self) -> String {
        (*self).into()
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
