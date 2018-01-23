
extern crate cryptocompare;
extern crate serde_json;

use cryptocompare::Options;
use std::time::Duration;
use std::thread;

#[test]
#[ignore]
fn test_get_coin_list() {
    let _result = cryptocompare::get_coin_list().unwrap();
}

#[test]
#[ignore]
fn test_get_price() {
    let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};

    loop {
        let result = cryptocompare::get_price("BTC", "USD", &option).unwrap();
        println!("{:?}", result.as_object().unwrap()["USD"].as_f64().unwrap());
        thread::sleep(Duration::from_secs(1))
    }
}

#[test]
#[ignore]
fn test_get_price_multi() {
    let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
    let _result = cryptocompare::get_price_multi("IOT", "USD,BTC,ETH", &option).unwrap();
}

#[test]
#[ignore]
fn test_get_price_multi_full() {
    let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
    let _result = cryptocompare::get_price_multi_full("IOT", "USD,BTC,ETH", &option);
}

#[test]
#[ignore]
fn test_get_price_historical() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let _result = cryptocompare::get_price_historical("IOT", "USD,BTC,ETH", &option);
}

#[test]
#[ignore]
fn test_get_history_day() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let limit: u64 = 10;
    let _result = cryptocompare::get_history_day("IOT", "USD", &option, &limit);
}

#[test]
#[ignore]
fn test_get_history_hour() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let limit: u64 = 10;
    let _result = cryptocompare::get_history_hour("IOT", "USD", &option, &limit);
}

#[test]
fn test_get_history_minute() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let limit: u64 = 1000;

    loop {
        let result = cryptocompare::get_history_minute("BTC", "USD", &option, &limit).unwrap();
        println!("{:#?}", result);
        thread::sleep(Duration::from_secs(60))
    }

    /*
    loop {
        let result = cryptocompare::get_history_minute("BTC", "USD", &option, &limit).unwrap();
        let data = result["Data"].as_array().unwrap();

        let open = &data[0]["open"].as_f64().unwrap();
        let close = &data[0]["close"].as_f64().unwrap();
        let high = &data[0]["high"].as_f64().unwrap();
        let low = &data[0]["low"].as_f64().unwrap();

        println!("open: {:?}; close: {:?}; high: {:?}; low: {:?}", open, close, high, low);
        thread::sleep(Duration::from_secs(60))
    }
    */
}