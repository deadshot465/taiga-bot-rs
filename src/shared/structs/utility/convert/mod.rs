#![allow(dead_code)]
pub mod conversion_table;
pub mod exchange_rate_api_response;
pub mod length;
pub mod temperature;
pub mod weight;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ConverterType {
    Length(length::Length, length::Length, f32),
    Weight(weight::Weight, weight::Weight, f32),
    Temperature(temperature::Temperature, temperature::Temperature, f32),
}

pub struct ParseConverterError;

pub trait FromStrToConverter {
    fn from_str_to_converter(
        _: &str,
        _: &str,
        _: f32,
    ) -> Result<ConverterType, ParseConverterError>;
}
