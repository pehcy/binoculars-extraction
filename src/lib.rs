use serde::de;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct KlineData {
    #[serde(deserialize_with = "de_int_from_str")]
    pub open: i32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub high: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub low: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub close: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub volume: f32,
    #[serde(deserialize_with = "de_int_from_str")]
    pub closetime: i32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub quote_asset_volume: f32,
    #[serde(deserialize_with = "de_int_from_str")]
    pub num_of_trades: i32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub taker_by_quote: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub taker_buy_quote: f32,
    #[serde(deserialize_with = "de_int_from_str")]
    pub ignore: i32
}

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

pub fn de_int_from_str<'a, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<i32>().map_err(de::Error::custom)
}