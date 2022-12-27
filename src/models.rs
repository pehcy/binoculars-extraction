use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use reqwest::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct OHLCVData {
    pub opentime: u64,

    #[serde(deserialize_with = "de_float_from_str")]
    pub open: f32,

    #[serde(deserialize_with = "de_float_from_str")]
    pub high: f32,

    #[serde(deserialize_with = "de_float_from_str")]
    pub low: f32,

    #[serde(deserialize_with = "de_float_from_str")]
    pub close: f32,

    #[serde(deserialize_with = "de_float_from_str")]
    pub volume: f32,

    pub closetime: u64,

    #[serde(deserialize_with = "de_float_from_str")]
    pub quote_asset_volume: f32,

    pub num_of_trades: u64,

    #[serde(deserialize_with = "de_float_from_str")]
    pub taker_by_quote: f32,

    #[serde(deserialize_with = "de_float_from_str")]
    pub taker_buy_quote: f32,

    #[serde(deserialize_with = "de_int_from_str")]
    pub ignore: i32
}

static BINANCE_ENDPOINT: &str = "https://api.binance.com/api/v3";

impl OHLCVData {
    pub async fn get(symbol: &String, interval: &String) -> Result<Vec<OHLCVData>, Error> {
        let url = format!(
            "{}/klines?symbol={}&interval={}&limit=500",
            BINANCE_ENDPOINT, symbol, interval
        );

        let res = reqwest::get(&url)
            .await?
            .json::<Vec<OHLCVData>>()
            .await?;

        Ok(res)
    }
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