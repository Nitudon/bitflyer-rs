use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, CoinInStatusType};
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getcoinins";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetCoinOutResponse {
    Error { errors: Vec<String> },
    Response (Vec<CoinOutInfo>)
}

#[derive(Deserialize, Debug)]
pub struct CoinOutInfo {
    pub id: u32,
    pub order_id: String,
    pub currency_code: CurrencyCode,
    pub amount: f32,
    pub address: String,
    pub tx_hash: String,
    pub fee: f32,
    pub additional_fee: f32,
    pub status: CoinInStatusType,
    pub event_date: String,
}

pub async fn get_coinouts(before: u32, after: u32, count: i32) -> Result<Vec<CoinOutInfo>, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(BEFORE_QUERY_KEY.to_string(), before.to_string());
    params.insert(AFTER_QUERY_KEY.to_string(), after.to_string());
    params.insert(COUNT_QUERY_KEY.to_string(), count.to_string());
    let response = api::get::<GetCoinOutResponse>(&PATH).await?;

    match response {
        GetCoinOutResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetCoinOutResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_coinouts::get_coinouts;

    #[tokio::test]
    async fn get_coinouts_test() {
        test_api!(get_coinouts(999, 0, 10));
    }
}
