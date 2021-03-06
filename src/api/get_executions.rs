use crate::api;
use crate::api::{ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY, COUNT_QUERY_KEY};
use std::collections::HashMap;

const PATH : &'static str = "/v1/executions";

type GetExecutionsResponse = Vec<ExecutionInfo>;

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

pub async fn get_executions(product_code: ProductCode, count: i32) -> Result<GetExecutionsResponse, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    params.insert(COUNT_QUERY_KEY.to_string(), count.to_string());
    api::get::<GetExecutionsResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_executions::get_executions;
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_executions_test() {
        test_api!(get_executions(ProductCode::BTC_JPY, 10));
    }
}
