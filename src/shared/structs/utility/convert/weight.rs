use crate::shared::structs::utility::convert::{
    ConverterType, FromStrToConverter, ParseConverterError,
};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Weight {
    Kilogram,
    Gram,
    Pound,
}

impl FromStr for Weight {
    type Err = ParseConverterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lowercase = s.trim().to_lowercase();
        if Self::all_available_units().contains(&lowercase.as_str()) {
            Ok(Self::new(s))
        } else {
            Err(ParseConverterError("Failed to parse weight unit."))
        }
    }
}

impl FromStrToConverter for Weight {
    fn from_str_to_converter(
        source: &str,
        target: &str,
        amount: f32,
    ) -> Result<ConverterType, ParseConverterError> {
        match (source.parse::<Weight>(), target.parse::<Weight>()) {
            (Ok(source_result), Ok(target_result)) => {
                Ok(ConverterType::Weight(source_result, target_result, amount))
            }
            _ => Err(ParseConverterError("Invalid weight unit.")),
        }
    }
}

impl ToString for Weight {
    fn to_string(&self) -> String {
        (*self).into()
    }
}

impl Into<&str> for Weight {
    fn into(self) -> &'static str {
        match self {
            Self::Kilogram => "kg",
            Self::Gram => "g",
            Self::Pound => "lb",
        }
    }
}

impl Into<String> for Weight {
    fn into(self) -> String {
        let str: &str = self.into();
        str.to_string()
    }
}

impl Weight {
    pub(self) fn new(source: &str) -> Self {
        match source {
            "kg" => Self::Kilogram,
            "g" => Self::Gram,
            "lb" => Self::Pound,
            _ => panic!("Invalid source text."),
        }
    }

    pub fn all_available_units() -> Vec<&'static str> {
        let all_units = vec![Self::Kilogram, Self::Gram, Self::Pound];
        all_units.into_iter().map(Into::<&str>::into).collect()
    }
}
