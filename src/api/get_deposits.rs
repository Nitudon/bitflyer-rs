use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, OrderStatusType};
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getdeposits";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetDepositsResponse {
    Error { errors: Vec<String> },
    Response (Vec<DepositInfo>)
}

#[derive(Deserialize, Debug)]
pub struct DepositInfo {
    pub id: u32,
    pub order_id: String,
    pub currency_code: CurrencyCode,
    pub amount: f32,
    pub status: OrderStatusType,
    pub event_date: String,
}

pub async fn get_deposits(before: u32, after: u32, count: i32) -> Result<Vec<DepositInfo>, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(BEFORE_QUERY_KEY.to_string(), before.to_string());
    params.insert(AFTER_QUERY_KEY.to_string(), after.to_string());
    params.insert(COUNT_QUERY_KEY.to_string(), count.to_string());
    let response = api::get::<GetDepositsResponse>(&PATH).await?;

    match response {
        GetDepositsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetDepositsResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_deposits::get_deposits;

    #[tokio::test]
    async fn get_deposits_test() {
        test_api!(get_deposits(999, 0, 10));
    }
}
