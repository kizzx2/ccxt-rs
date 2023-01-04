# ccxt-rs

A transpilation of the CCXT library into Rust with proper parsing.

This is a proof of concept and **work in progress**, `fetch_balance`, `fetch_order_book` and `create_order` have been tested to work with Binance.

If you want to use this, check out the repository and then put the `rust` directory into your project.

### Help wanted

As mentioned above this is a work in progress, if you bump into any issues in your use case please feel free to reach out by creating a GitHub issue and I can help you get up to speed on the code and working on a patch.

### Example usage

```rust
use ccxt::exchange::{Exchange, Value, normalize};
use ccxt::binance::{Binance, BinanceImpl};

use serde_json::json;

const UNDEFINED: Value = Value::Undefined;

#[tokio::main]
async fn main() {
    let mut b = BinanceImpl::new(Value::Json(json!({
        "apiKey": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
        "secret": "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    })));
    // b.set_sandbox_mode(true);

    let rv = Binance::fetch_order_book(&mut b, "BTC/USDT".into(), UNDEFINED, UNDEFINED).await;
    println!("{}", normalize(&rv).unwrap());

    let rv = Binance::fetch_balance(&mut b, UNDEFINED).await;
    println!("{}", normalize(&rv).unwrap());

    let rv = Binance::create_order(&mut b, "BTC/USDT".into(), "limit".into(), "buy".into(), 0.001.into(), 16789.2.into(), UNDEFINED).await;
    println!("{}", normalize(&rv).unwrap());
}
```