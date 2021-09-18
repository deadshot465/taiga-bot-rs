use crate::shared::structs::utility::convert::{
    ConverterType, FromStrToConverter, ParseConverterError,
};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl FromStr for Temperature {
    type Err = ParseConverterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.trim().to_lowercase();
        if Self::all_available_units().contains(&lowercase.as_str()) {
            Ok(Self::new(s))
        } else {
            Err(ParseConverterError("Failed to parse temperature unit."))
        }
    }
}

impl FromStrToConverter for Temperature {
    fn from_str_to_converter(
        source: &str,
        target: &str,
        amount: f32,
    ) -> Result<ConverterType, ParseConverterError> {
        match (source.parse::<Temperature>(), target.parse::<Temperature>()) {
            (Ok(source_result), Ok(target_result)) => Ok(ConverterType::Temperature(
                source_result,
                target_result,
                amount,
            )),
            _ => Err(ParseConverterError("Invalid temperature unit.")),
        }
    }
}

impl ToString for Temperature {
    fn to_string(&self) -> String {
        (*self).into()
    }
}

impl Into<&str> for Temperature {
    fn into(self) -> &'static str {
        match self {
            Self::Celsius => "c",
            Self::Fahrenheit => "f",
            Self::Kelvin => "k",
        }
    }
}

impl Into<String> for Temperature {
    fn into(self) -> String {
        let str: &str = self.into();
        str.to_string()
    }
}

impl Temperature {
    pub(self) fn new(source: &str) -> Self {
        match source {
            "c" => Self::Celsius,
            "f" => Self::Fahrenheit,
            "k" => Self::Kelvin,
            _ => panic!("Invalid source text."),
        }
    }

    pub fn all_available_units() -> Vec<&'static str> {
        let all_units = vec![Self::Celsius, Self::Fahrenheit, Self::Kelvin];
        all_units.into_iter().map(Into::<&str>::into).collect()
    }
}
