extern crate curl;
extern crate serde_json;
extern crate sha2;
extern crate hmac;

mod httpclient;

use httpclient::{Bittrex, Public, Market, Account};

fn main() {
    let mut bittrex = Bittrex::new();

    //public methods
//    let public_data = bittrex.get_markets().unwrap();
//    println!("{:?}", public_data);
//    let public_data = bittrex.get_currencies().unwrap();
//    println!("{:?}", public_data);
//    let public_data = bittrex.get_ticker("BTC-LTC").unwrap();
//    println!("{:?}", public_data);
//    let public_data = bittrex.get_market_summaries().unwrap();
//    println!("{:?}", public_data);
    let public_data = bittrex.get_order_book("BTC-LTC", "both").unwrap();
    println!("{:?}", public_data);
//    let public_data = bittrex.get_market_history("BTC-LTC").unwrap();
//    println!("{:?}", public_data);

    //market methods
//    let market = bittrex.buy_limit("USDT-ETH", 0.1, 0.1).unwrap();
//    println!("{:?}", market);
//    let market = bittrex.sell_limit("USDT-ETH", 0.1, 0.1).unwrap();
//    println!("{:?}", market);
//    let market = bittrex.cancel("614c34e4-8d71-11e3-94b5-425861b86ab6").unwrap();
//    println!("{:?}", market);
    let market = bittrex.get_open_orders("USDT-ETH").unwrap();
    println!("{:?}", market);

    //account methods
    let account = bittrex.get_deposit_history("").unwrap();
    println!("{:?}", account);
    let account = bittrex.get_order_history("USDT-BTG").unwrap();
    println!("{:?}", account);
    let account = bittrex.get_order("207b559c-476c-4689-b6d7-e4b62b4831ea").unwrap();
    println!("{:?}", account);
    let account = bittrex.get_deposit_address("BTC").unwrap();
    println!("{:?}", account);
    let account = bittrex.get_balance("BTC").unwrap();
    println!("{:?}", account);
    let account = bittrex.get_balances().unwrap();
    println!("{:?}", account);
    let account = bittrex.withdraw("ETH", 0.1, "0x3315C11aC4c20250109830267FeD98626bB979Dc").unwrap();
    println!("{:?}", account);
}
