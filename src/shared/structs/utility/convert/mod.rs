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
    Currency(f32),
}

pub struct ParseConverterError(&'static str);

pub trait FromStrToConverter {
    fn from_str_to_converter(
        _: &str,
        _: &str,
        _: f32,
    ) -> Result<ConverterType, ParseConverterError>;
}

impl ConverterType {
    pub fn new(source_unit: &str, target_unit: &str, amount: f32) -> Self {
        length::Length::from_str_to_converter(source_unit, target_unit, amount)
            .or_else(|_| weight::Weight::from_str_to_converter(source_unit, target_unit, amount))
            .or_else(|_| {
                temperature::Temperature::from_str_to_converter(source_unit, target_unit, amount)
            })
            .unwrap_or(ConverterType::Currency(amount))
    }
}
