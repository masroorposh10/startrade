use reqwest::Client;
use serde_json::{json, Value};
use dotenv::dotenv;
use std::env;

pub async fn get_ai_recommendation(market_data: &str) -> Result<Value, reqwest::Error> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let client = Client::new();
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&json!({
            "model": "gpt-4",
            "messages": [
                {"role": "system", "content": "You are an expert trading advisor."},
                {"role": "user", "content": format!("Based on this market data: {}. What is your trade recommendation? (BUY/SELL/HOLD)", market_data)}
            ],
            "max_tokens": 50,
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(response)
}
