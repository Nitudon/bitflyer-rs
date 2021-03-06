use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, OrderStatusType};
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getcoinins";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetCoinInResponse {
    Error { errors: Vec<String> },
    Response (Vec<CoinInInfo>)
}

#[derive(Deserialize, Debug)]
pub struct CoinInInfo {
    pub id: u32,
    pub order_id: String,
    pub currency_code: CurrencyCode,
    pub amount: f32,
    pub address: String,
    pub tx_hash: String,
    pub status: OrderStatusType,
    pub event_date: String,
}

pub async fn get_coinins(before: u32, after: u32, count: i32) -> Result<Vec<CoinInInfo>, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(BEFORE_QUERY_KEY.to_string(), before.to_string());
    params.insert(AFTER_QUERY_KEY.to_string(), after.to_string());
    params.insert(COUNT_QUERY_KEY.to_string(), count.to_string());
    let response = api::get::<GetCoinInResponse>(&PATH).await?;

    match response {
        GetCoinInResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetCoinInResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_coinins::get_coinins;

    #[tokio::test]
    async fn get_coinins_test() {
        test_api!(get_coinins(999, 0, 10));
    }
}
