
extern crate cryptocompare;
extern crate serde_json;

use cryptocompare::Options;


#[test]
fn test_get_coin_list() {

    let result = cryptocompare::get_coin_list();
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_get_price() {
    let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
    let result = cryptocompare::get_price("BTC", "USD", &option);
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_get_price_multi() {
    let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
    let result = cryptocompare::get_price_multi("IOT", "USD,BTC,ETH", &option);
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_get_price_multi_full() {
    let option: Options = Options {exchanges: "Bitfinex", try_conversion: false};
    let result = cryptocompare::get_price_multi_full("IOT", "USD,BTC,ETH", &option);
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_get_price_historical() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let result = cryptocompare::get_price_historical("IOT", "USD,BTC,ETH", &option);
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_get_history_day() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let limit: u64 = 3;
    let result = cryptocompare::get_history_day("BTC", "USD", &option, &limit);
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_get_history_hour() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let limit: u64 = 3;
    let result = cryptocompare::get_history_hour("BTC", "USD", &option, &limit);
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_get_history_minute() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let limit: u64 = 3;
    let result = cryptocompare::get_history_minute("BTC", "USD", &option, &limit);
    assert_eq!(result.is_err(), false)
}

#[test]
fn test_parse_json_to_vector() {
    let option: Options = Options { exchanges: "Bitfinex", try_conversion: false };
    let limit: u64 = 3;
    let result = cryptocompare::get_history_minute("BTC", "USD", &option, &limit).unwrap();
    let vector = cryptocompare::parse_json_to_vector(result).unwrap();
    assert_eq!(vector.len(), (limit + 1) as usize)
}

#[test]
fn test_parse_json_to_float() {
    let option: Options = Options {exchanges: "Bitfinex", try_conversion: true};
    let result = cryptocompare::get_price("IOT", "ETH", &option).unwrap();
    let price = cryptocompare::parse_json_to_float(result, "ETH");
    assert_eq!(price.is_err(), false);
}
