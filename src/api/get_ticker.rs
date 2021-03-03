use crate::api;
use crate::api::{ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use chrono::NaiveDateTime;
use std::collections::HashMap;

const PATH : &'static str = "/v1/ticker";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetTickerResponse {
    Error { errors: Vec<String> },
    Response {     
        product_code: String,
        timestamp: String,
        tick_id: u32,
        best_bid: f32,
        best_ask: f32,
        best_bid_size: f32,
        best_ask_size: f32,
        total_bid_depth: f32,
        total_ask_depth: f32,
        ltp: f32,
        volume: f32,
        volume_by_product: f32
    }
}

#[derive(Deserialize, Debug)]
pub struct TickerInfo {
    pub product_code: String,
    pub timestamp: NaiveDateTime,
    pub tick_id: u32,
    pub best_bid: f32,
    pub best_ask: f32,
    pub best_bid_size: f32,
    pub best_ask_size: f32,
    pub total_bid_depth: f32,
    pub total_ask_depth: f32,
    pub ltp: f32,
    pub volume: f32,
    pub volume_by_product: f32
}

pub async fn get_ticker(product_code: ProductCode) -> Result<TickerInfo, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    let response = api::get::<GetTickerResponse>(&PATH).await?;

    match response {
        GetTickerResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetTickerResponse::Response { product_code, timestamp, tick_id, best_bid, best_ask, best_bid_size, best_ask_size, total_bid_depth, total_ask_depth, ltp, volume, volume_by_product } => Ok(TickerInfo{
            product_code,
            timestamp : NaiveDateTime::parse_from_str(&timestamp, "%Y-%m-%dT%H:%M:%S%.f").unwrap(),
            tick_id,
            best_bid,
            best_ask,
            best_bid_size,
            best_ask_size,
            total_bid_depth,
            total_ask_depth,
            ltp,
            volume,
            volume_by_product
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_ticker::get_ticker;
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_ticker_test() {
        test_api!(get_ticker(ProductCode::BTC_JPY));
    }
}
