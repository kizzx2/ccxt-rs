use ccxt::exchange::{Exchange, Value, normalize, ValueTrait};
use ccxt::binance::{Binance, BinanceImpl};

use serde_json::json;

const UNDEFINED: Value = Value::Undefined;



async fn get_order_book(b: &mut BinanceImpl) {
    let rv = Binance::fetch_order_book(b, "BTC/USDT".into(), UNDEFINED, UNDEFINED).await;
    println!("{}", normalize(&rv).unwrap());
}

async fn get_balance(b: &mut BinanceImpl) {
    let rv = Binance::fetch_balance(b, UNDEFINED).await;
    println!("2 {}", normalize(&rv).unwrap());
}


async fn create_order(b: &mut BinanceImpl) {
    let rv = Binance::create_order(b, "BTC/USDT".into(), "limit".into(), "buy".into(), 0.001.into(), 16789.2.into(), UNDEFINED).await;
    println!("3 {}", normalize(&rv).unwrap());
}

#[tokio::main]
async fn main() {
    let mut b = BinanceImpl::new(Value::Json(json!({
        "apiKey": "zby7nbyy8NML4UEXzFNbyJ6yPBtjWFSlH2tedU0AacgONqsGgCBgiMbr0QXPElas",
        "secret": "lVmuLcWBcla7w00QgCFitmbk0YldYPvtZQeuLonn3ddlP9I8KvfDr6bVSRTsGEUZ",
    })));
    b.set_sandbox_mode(true);

    get_order_book(&mut b).await;
    get_balance(&mut b).await;
    create_order(&mut b).await;
}