
extern crate reqwest;
extern crate serde_json;
extern crate time;

use serde_json::Value;
use std::io::Read;

static BASE_URL: &str = "https://min-api.cryptocompare.com/data/";

pub struct Options<'a> {
    pub exchanges: &'a str,
    pub try_conversion: bool
}

pub fn get_coin_list() -> Result<serde_json::Value, Box<::std::error::Error>> {
    let url = format!("{}all/coinlist", BASE_URL);

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

pub fn get_price(fsym: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}price?fsym={}&tsyms={}", BASE_URL, fsym, tsyms);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

pub fn get_price_multi(fsyms: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}pricemulti?fsyms={}&tsyms={}", BASE_URL, fsyms, tsyms);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

pub fn get_price_multi_full(fsyms: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}pricemultifull?fsyms={}&tsyms={}", BASE_URL, fsyms, tsyms);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

pub fn get_price_historical(fsym: &str, tsyms: &str, options: &Options) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}pricehistorical?fsym={}&tsyms={}&ts={}", BASE_URL, fsym, tsyms, time::get_time().sec);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

pub fn get_history_day(fsym: &str, tsym: &str, options: &Options, limit: &u64) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}histoday?fsym={}&tsym={}&limit={}&aggregate=1", BASE_URL, fsym, tsym, limit);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

pub fn get_history_hour(fsym: &str, tsym: &str, options: &Options, limit: &u64) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}histohour?fsym={}&tsym={}&limit={}&aggregate=1", BASE_URL, fsym, tsym, limit);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}

pub fn get_history_minute(fsym: &str, tsym: &str, options: &Options, limit: &u64) -> Result<serde_json::Value, Box<::std::error::Error>> {
    let mut url = format!("{}histominute?fsym={}&tsym={}&limit={}&aggregate=1", BASE_URL, fsym, tsym, limit);
    if options.exchanges != "" { url = format!("{}&e={}", url, options.exchanges); }
    if !options.try_conversion { url = format!("{}&tryConversion={}", url, options.try_conversion); }

    let mut result = String::new();
    reqwest::get(&url)?.read_to_string(&mut result)?;
    let value: Value = serde_json::from_str(&mut result)?;

    Ok(value)
}