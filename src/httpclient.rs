use curl::easy::{Easy, List};
use serde_json;
use serde_json::{Value, Error};
use sha2::Sha512;
use hmac::{Hmac, Mac};

pub struct Bittrex {
    api_key: String,
    secret: String,
    set_sign: bool,
}


//const (
//bittrexMaxOpenOrders       = 500
//bittrexMaxOrderCountPerDay = 200000
//
//// Returned messages from Bittrex API
//bittrexAddressGenerating      = "ADDRESS_GENERATING"
//bittrexErrorMarketNotProvided = "MARKET_NOT_PROVIDED"
//bittrexErrorInvalidMarket     = "INVALID_MARKET"
//bittrexErrorAPIKeyInvalid     = "APIKEY_INVALID"
//bittrexErrorInvalidPermission = "INVALID_PERMISSION"
//)

impl Bittrex {
    pub fn new() -> Self {
        Bittrex {
            api_key: "6cde5ce871174660a5e496d55c1d763c".to_string(),
            secret: "01eb40e18539435abae46f2f11af9a87".to_string(),
            set_sign: false
        }
    }

    pub fn request(&self, url: &str) -> Result<Value, Error> {
        let mut buf = Vec::new();
        let mut handler = Easy::new();
        let url = if self.set_sign == false {
            format!("https://bittrex.com/api/v1.1/{}", url)
        } else {
            let u = format!("https://bittrex.com/api/v1.1/{}&nonce={}", url, "1000");
            handler.http_headers(self.add_sign(u.as_ref()).unwrap()).unwrap();
//            handler.verbose(true).unwrap();
            u
        };

        handler.url(url.as_ref()).unwrap();
        {
            let mut transfer = handler.transfer();
            transfer.write_function(|new_data| {
                buf.extend_from_slice(new_data);
                Ok(new_data.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        Ok(serde_json::from_slice(&buf).unwrap())
    }


//    $apikey='xxx';
//    $apisecret='xxx';
//    $nonce=time();
//    $uri='https://bittrex.com/api/v1.1/market/getopenorders?apikey='.$apikey.'&nonce='.$nonce;
//    $sign=hash_hmac('sha512',$uri,$apisecret);
//    $ch = curl_init($uri);
//    curl_setopt($ch, CURLOPT_HTTPHEADER, array('apisign:'.$sign));
//    $execResult = curl_exec($ch);
//    $obj = json_decode($execResult);

    fn add_sign(&self, url: &str) -> Result<List, Error> {
        let mut hmac = Hmac::<Sha512>::new(self.secret.as_bytes()).unwrap();
        hmac.input(url.as_bytes());
        let sign = hmac.result().code();
        let header_api_sign = format!("apisign: {:X}", sign);
        let mut list = List::new();
        list.append(header_api_sign.as_ref()).unwrap();
        Ok(list)
    }

}

//// Public requests
//bittrexAPIGetMarkets         = "public/getmarkets"
//bittrexAPIGetCurrencies      = "public/getcurrencies"
//bittrexAPIGetTicker          = "public/getticker"
//bittrexAPIGetMarketSummaries = "public/getmarketsummaries"
//bittrexAPIGetMarketSummary   = "public/getmarketsummary"
//bittrexAPIGetOrderbook       = "public/getorderbook"
//bittrexAPIGetMarketHistory   = "public/getmarkethistory"
pub trait Public {
    fn get_markets(&mut self) -> Result<Value, Error>;
    fn get_currencies(&mut self) -> Result<Value, Error>;
    fn get_ticker(&mut self, market: &str) -> Result<Value, Error>;
    fn get_market_summaries(&mut self) -> Result<Value, Error>;
    fn get_market_summary(&mut self, market: &str) -> Result<Value, Error>;
    fn get_order_book(&mut self, market: &str, m_type: &str) -> Result<Value, Error>;
    fn get_market_history(&mut self, market: &str) -> Result<Value, Error>;
}

impl Public for Bittrex {
    fn get_markets(&mut self) -> Result<Value, Error> {
        self.set_sign = false;
        self.request("public/getmarkets")
    }

    fn get_currencies(&mut self) -> Result<Value, Error> {
        self.set_sign = false;
        self.request("public/getcurrencies")
    }

    fn get_ticker(&mut self, market: &str) -> Result<Value, Error> {
        self.set_sign = false;
        let path = format!("public/getticker?market={}", market);
        self.request(path.as_ref())
    }

    fn get_market_summaries(&mut self) -> Result<Value, Error> {
        self.set_sign = false;
        self.request("public/getmarketsummaries")
    }

    fn get_market_summary(&mut self, market: &str) -> Result<Value, Error> {
        self.set_sign = false;
        let path = format!("public/getmarketsummary?market={}", market);
        self.request(path.as_ref())
    }

    fn get_order_book(&mut self, market: &str, m_type: &str) -> Result<Value, Error> {
        self.set_sign = false;
        let path = format!("public/getmarketsummary?market={}&type={}", market, m_type);
        self.request(path.as_ref())
    }

    fn get_market_history(&mut self, market: &str) -> Result<Value, Error> {
        self.set_sign = false;
        let path = format!("public/getmarketsummary?market={}", market);
        self.request(path.as_ref())
    }
}


//// Market requests
//bittrexAPIBuyLimit      = "market/buylimit"
//bittrexAPISellLimit     = "market/selllimit"
//bittrexAPICancel        = "market/cancel"
//bittrexAPIGetOpenOrders = "market/getopenorders"
pub trait Market {
    fn buy_limit(&mut self, market: &str, qty: f32, rate: f32) -> Result<Value, Error>;
    fn sell_limit(&mut self, market: &str, qty: f32, rate: f32) -> Result<Value, Error>;
    fn cancel(&mut self, uuid: &str) -> Result<Value, Error>;
    fn get_open_orders(&mut self, market: &str) -> Result<Value, Error>;
}

impl Market for Bittrex {

//    parameter	required	description
//
//    market	required	a string literal for the market (ex: BTC-LTC)
//    quantity	required	the amount to purchase
//    rate	    required	the rate at which to place the order.

    fn buy_limit(&mut self, market: &str, qty: f32, rate: f32) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("market/buylimit?apikey={}&market={}&quantity={}&rate={}", self.api_key, market, qty, rate);
        self.request(path.as_ref())
    }

    fn sell_limit(&mut self, market: &str, qty: f32, rate: f32) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("market/selllimit?apikey={}&market={}&quantity={}&rate={}", self.api_key, market, qty, rate);
        self.request(path.as_ref())
    }

    fn cancel(&mut self, uuid: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("market/cancel?apikey={}&uuid={}", self.api_key, uuid);
        self.request(path.as_ref())
    }

    fn get_open_orders(&mut self, market: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("market/cancel?apikey={}&market={}", self.api_key, market);
        self.request(path.as_ref())
    }
}


//// Account requests
//bittrexAPIGetBalances          = "account/getbalances"
//bittrexAPIGetBalance           = "account/getbalance"
//bittrexAPIGetDepositAddress    = "account/getdepositaddress"
//bittrexAPIWithdraw             = "account/withdraw"
//bittrexAPIGetOrder             = "account/getorder"
//bittrexAPIGetOrderHistory      = "account/getorderhistory"
//bittrexAPIGetWithdrawalHistory = "account/getwithdrawalhistory"
//bittrexAPIGetDepositHistory    = "account/getdeposithistory"
pub trait Account {
    fn get_balances(&mut self) -> Result<Value, Error>;
    fn get_balance(&mut self, currency: &str) -> Result<Value, Error>;
    fn get_deposit_address(&mut self, currency: &str) -> Result<Value, Error>;
    fn withdraw(&mut self, currency: &str, qty: f32, address: &str) -> Result<Value, Error>;
    fn get_order(&mut self, uuid: &str) -> Result<Value, Error>;
    fn get_order_history(&mut self, market: &str) -> Result<Value, Error>;
    fn get_withdraw_history(&mut self, currency: &str) -> Result<Value, Error>;
    fn get_deposit_history(&mut self, currency: &str) -> Result<Value, Error>;
}

impl Account for Bittrex {
    fn get_balances(&mut self) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("account/getbalances?apikey={}", self.api_key);
        self.request(path.as_ref())
    }

    fn get_balance(&mut self, currency: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("account/getbalance?apikey={}&currency={}", self.api_key, currency);
        self.request(path.as_ref())
    }

    fn get_deposit_address(&mut self, currency: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("account/getdepositaddress?apikey={}&currency={}", self.api_key, currency);
        self.request(path.as_ref())
    }

    fn withdraw(&mut self, currency: &str, qty: f32, address: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("account/withdraw?apikey={}&currency={}&quantity={}&address={}", self.api_key, currency, qty, address);
        self.request(path.as_ref())
    }

    fn get_order(&mut self, uuid: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = format!("account/getorder?apikey={}&uuid={}", self.api_key, uuid);
        self.request(path.as_ref())
    }

    fn get_order_history(&mut self, market: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = if market == "" {
            format!("account/getorderhistory?apikey={}", self.api_key)
        } else {
            format!("account/getorderhistory?apikey={}&market={}", self.api_key, market)
        };
        self.request(path.as_ref())
    }

    fn get_withdraw_history(&mut self, currency: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = if currency == "" {
            format!("account/getwithdrawalhistory?apikey={}", self.api_key)
        } else {
            format!("account/getwithdrawalhistory?apikey={}&currency={}", self.api_key, currency)
        };
        self.request(path.as_ref())
    }

    fn get_deposit_history(&mut self, currency: &str) -> Result<Value, Error> {
        self.set_sign = true;
        let path = if currency == "" {
            format!("account/getdeposithistory?apikey={}", self.api_key)
        } else {
            format!("account/getdeposithistory?apikey={}&currency={}", self.api_key, currency)
        };
        self.request(path.as_ref())
    }
}