
#![feature(try_trait)]

//!
//! # General
//!
//! An rust API-Wrapper for the CryptoCompare API.
//!
//! Link to www.cryptocompare.com/api/ [API](https://www.cryptocompare.com/api/).
//!

extern crate reqwest;
extern crate serde_json;
extern crate time;

use serde_json::Value;
use std::io::Read;

static BASE_URL: &str = "https://min-api.cryptocompare.com/data/";

///
/// Informations for the request.
///
pub struct Options<'a> {
    /// Exchange the request should go to. List is at [API](https://www.cryptocompare.com/api/)
    pub exchanges: &'a str,
    /// If the value should be tried to convert to BTC or not.
    pub try_conversion: bool
}

///
/// An object with trading info.
///
#[derive(Debug)]
pub struct Dataset {
    /// Market price when timeslot closed.
    pub close: f64,
    /// Highest market price in timeslot.
    pub high: f64,
    /// LOwest market price in timeslot.
    pub low: f64,
    /// Market price when time slot opened.
    pub open: f64,
    /// Time in seconds after 1.1.1970.
    pub time: i64,
    /// Start volume.
    pub volumefrom: f64,
    /// End volume.
    pub volumeto: f64
}

///
/// Get general info for all the coins available via CryptoCompare.
///
/// # Example
/// ```
/// let coin_list = cryptocompare::get_coin_list()
/// ```
///
pub fn get_coin_list() -> Result<serde_json::Value, Box<::std::error::Error>> {
    let url = format!("{}all/coinlist", BASE_URL);

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Get the latest price for a list of one or more currencies.
///
/// # Example
/// ```
///     let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
///     let price = cryptocompare::get_price("BTC", "USD", &option);
/// ```
///
pub fn get_price(fsym: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}price?fsym={}&tsyms={}", BASE_URL, fsym, tsyms);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Get a matrix of currency prices.
///
/// # Example
/// ```
///     let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
///     let price_multi = cryptocompare::get_price_multi("IOT", "USD,BTC,ETH", &option);
/// ```
///
pub fn get_price_multi(fsyms: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}pricemulti?fsyms={}&tsyms={}", BASE_URL, fsyms, tsyms);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Get all the current trading info of any list of cryptocurrencies in any other currency that you need.
///
/// If the crypto does not trade directly into the toSymbol requested, BTC will be used for conversion
///
/// # Example
/// ```
///     let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
///     let price_multi_full = cryptocompare::get_price_multi_full("IOT", "USD,BTC,ETH", &option);
/// ```
///
pub fn get_price_multi_full(fsyms: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}pricemultifull?fsyms={}&tsyms={}", BASE_URL, fsyms, tsyms);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Get the price of any cryptocurrency in any other currency that you need at a given timestamp.
///
/// The price comes from the daily info - so it would be the price at the end of the day GMT based on the requested TS.
/// If the crypto does not trade directly into the toSymbol requested, BTC will be used for conversion.
/// Tries to get direct trading pair data, if there is none or it is more than 30 days before the ts requested, it uses BTC conversion.
///
/// # Example
/// ```
///     let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
///     let price_historical = cryptocompare::get_price_historical("IOT", "USD,BTC,ETH", &option);
/// ```
///
pub fn get_price_historical(fsym: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}pricehistorical?fsym={}&tsyms={}&ts={}", BASE_URL, fsym, tsyms, time::get_time().sec);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Get open, high, low, close, volumefrom and volumeto daily historical data.
///
/// The values are based on 00:00 GMT time.
/// It uses BTC conversion if data is not available because the coin is not trading in the specified currency.
///
/// # Example
/// ```
///     let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
///     let limit: u64 = 3;
///     let history_day = cryptocompare::get_history_day("BTC", "USD", &option, &limit);
/// ```
///
pub fn get_history_day(fsym: &str, tsym: &str, options: &Options, limit: &u64) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}histoday?fsym={}&tsym={}&limit={}&aggregate=1", BASE_URL, fsym, tsym, limit);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Get open, high, low, close, volumefrom and volumeto from the each hour historical data.
///
/// It uses BTC conversion if data is not available because the coin is not trading in the specified currency.
///
/// # Example
/// ```
///     let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
///     let limit: u64 = 3;
///     let history_hour = cryptocompare::get_history_hour("BTC", "USD", &option, &limit);
/// ```
///
pub fn get_history_hour(fsym: &str, tsym: &str, options: &Options, limit: &u64) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}histohour?fsym={}&tsym={}&limit={}&aggregate=1", BASE_URL, fsym, tsym, limit);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Get open, high, low, close, volumefrom and volumeto from the each minute historical data.
///
/// This data is only stored for 7 days, if you need more,use the hourly or daily path.
/// It uses BTC conversion if data is not available because the coin is not trading in the specified currency.
///
/// # Example
/// ```
///     let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
///     let limit: u64 = 3;
///     let history_minute = cryptocompare::get_history_minute("BTC", "USD", &option, &limit);
/// ```
///
pub fn get_history_minute(fsym: &str, tsym: &str, options: &Options, limit: &u64) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}histominute?fsym={}&tsym={}&limit={}&aggregate=1", BASE_URL, fsym, tsym, limit);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

///
/// Helper function to parse a JSON-Value to a Vector of Datasets
///
/// # Example
/// ```
///     let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
///     let limit: u64 = 3;
///     let history_minute = cryptocompare::get_history_minute("BTC", "USD", &option, &limit).unwrap();
///     let vector = cryptocompare::parse_json_to_vector(history_minute).unwrap();
/// ```
///
pub fn parse_json_to_vector(object: Value) -> Result<Vec<Dataset>, Box<::std::option::NoneError>> {
    let mut vector: Vec<Dataset> = Vec::new();
    let array = object.as_object()?["Data"].as_array()?;

    for entry in array {
        let elem = entry.as_object()?;
        vector.push(Dataset {
            close: elem["close"].as_f64()?,
            high: elem["high"].as_f64()?,
            low: elem["low"].as_f64()?,
            open: elem["open"].as_f64()?,
            time: elem["time"].as_i64()?,
            volumefrom: elem["volumefrom"].as_f64()?,
            volumeto: elem["volumeto"].as_f64()?
        })
    }
    Ok(vector)
}