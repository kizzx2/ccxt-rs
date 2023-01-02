use ccxt::exchange::{Exchange, Value, normalize, ValueTrait};
use ccxt::binance::{Binance, BinanceImpl};

use serde_json::json;

const UNDEFINED: Value = Value::Undefined;

#[tokio::main]
async fn main() {
    let mut b = BinanceImpl::new(Value::Json(json!({
        // "apiKey": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        // "secret": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    })));
    // b.set_sandbox_mode(true);

    let rv = Binance::fetch_order_book(&mut b, "BTC/USDT".into(), UNDEFINED, UNDEFINED).await;
    println!("{}", normalize(&rv).unwrap());
}