use ccxt::exchange::{Exchange, Value, normalize, ValueTrait};
use ccxt::binance::{Binance, BinanceImpl};

use serde_json::json;

const UNDEFINED: Value = Value::Undefined;

#[tokio::main]
async fn main() {
    let mut b = BinanceImpl::new();

    // let rv = Binance::fetch_time(&mut b, UNDEFINED).await;
    // println!("{}", normalize(&rv).unwrap());
    //
    // let rv = Binance::fetch_status(&mut b, UNDEFINED).await;
    // println!("{}", normalize(&rv).unwrap());
    //
    // // let rv = Binance::fetch_order_book(&mut b, "BTC/USDT".into(), UNDEFINED, UNDEFINED).await;
    // // println!("{}", normalize(&rv).unwrap());
    //

    let rv = Binance::fetch_order_book(&mut b, "BTC/USDT".into(), UNDEFINED, UNDEFINED).await;
    println!("{}", normalize(&rv).unwrap());
}