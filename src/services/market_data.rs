use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TickerPrice {
    pub symbol: String,
    pub price: String,
}

pub async fn fetch_binance_price(symbol: &str) -> Result<TickerPrice, reqwest::Error> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let resp = reqwest::get(&url).await?.json::<TickerPrice>().await?;
    Ok(resp)
}
