use crate::api;
use crate::api::{ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use chrono::NaiveDateTime;
use std::collections::HashMap;

const PATH : &'static str = "/v1/ticker";

type GetTickerResponse = TickerInfo;

#[derive(Deserialize, Debug)]
pub struct TickerInfo {
    pub product_code: String,
    pub timestamp: String,
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

impl TickerInfo {
    pub fn get_naive_date_timestamp(&self) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(&self.timestamp, "%Y-%m-%dT%H:%M:%S%.f").unwrap()
    } 
}

pub async fn get_ticker(product_code: ProductCode) -> Result<GetTickerResponse, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    api::get::<GetTickerResponse>(&PATH).await
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
