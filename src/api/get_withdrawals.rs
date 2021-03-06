use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, OrderStatusType, MESSAGE_ID_QUERY_KEY};
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getwithdrawals";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetWithdrawalsResponse {
    Error { errors: Vec<String> },
    Response (Vec<WithdrawalsInfo>)
}

#[derive(Deserialize, Debug)]
pub struct WithdrawalsInfo {
    pub id: u32,
    pub order_id: String,
    pub currency_code: CurrencyCode,
    pub amount: f32,
    pub status: OrderStatusType,
    pub event_date: String,
}

pub async fn get_withdrawals(before: u32, after: u32, count: i32, message_id: Option<String>) -> Result<Vec<WithdrawalsInfo>, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(BEFORE_QUERY_KEY.to_string(), before.to_string());
    params.insert(AFTER_QUERY_KEY.to_string(), after.to_string());
    params.insert(COUNT_QUERY_KEY.to_string(), count.to_string());
    if message_id.is_some() {
        params.insert(MESSAGE_ID_QUERY_KEY.to_string(), message_id.unwrap());
    }
    let response = api::get::<GetWithdrawalsResponse>(&PATH).await?;

    match response {
        GetWithdrawalsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetWithdrawalsResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_withdrawals::get_withdrawals;

    #[tokio::test]
    async fn get_deposits_test() {
        test_api!(get_withdrawals(999, 0, 10, None));
    }
}
