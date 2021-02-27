use crate::api;
use crate::api::{ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY, COUNT_QUERY_KEY};
use std::collections::HashMap;

const PATH : &'static str = "/v1/executions";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetExecutionsResponse {
    Error { errors: Vec<String> },
    Response (Vec<ExecutionInfo>)
}

#[derive(Deserialize, Debug)]
pub struct ExecutionInfo {
    pub id: u32,
    pub side: String,
    pub price: f32,
    pub size: f32,
    pub exec_date: String,
    pub buy_child_order_acceptance_id: String,
    pub sell_child_order_acceptance_id: String,
}

pub async fn get_executions(product_code: ProductCode, count: i32) -> Result<Vec<ExecutionInfo>, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    params.insert(COUNT_QUERY_KEY.to_string(), count.to_string());
    let response = api::get::<GetExecutionsResponse>(&PATH).await?;

    match response {
        GetExecutionsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetExecutionsResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_executions::get_executions;
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_executions_test() {
        let response = get_executions(ProductCode::BTC_JPY, 10).await;
        assert_eq!(response.is_ok(), true)
    }
}
