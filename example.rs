
extern crate cryptocompare;

use cryptocompare::*;
use std::time::Duration;
use std::thread::*;

fn main() {
    let option = Options {exchanges: "CCCAGG", try_conversion: false};
    let limit: u64 = 1;

    loop {
        sleep(Duration::from_secs(15));

        let raw_price = get_history_minute("ETH", "BTC", &option, &limit).unwrap();

        let price = parse_json_to_vector(raw_price).unwrap();

        println!("time=>{:?} open:{:?}; low:{:?}; high:{:?}; close:{:?}",
                 price[0].time,
                 price[0].open,
                 price[0].low,
                 price[0].high,
                 price[0].close);
    }
}

