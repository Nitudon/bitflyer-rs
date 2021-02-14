use crate::api;
use crate::api::ApiResponseError;

const METHOD : &'static str = "executions";

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

pub async fn get_executions() -> Result<Vec<ExecutionInfo>, ApiResponseError> {
    let response = api::get::<GetExecutionsResponse>(&METHOD).await?;

    match response {
        GetExecutionsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetExecutionsResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_executions::get_executions;

    #[tokio::test]
    async fn get_executions_test() {
        let response = get_executions().await;
        assert_eq!(response.is_ok(), true)
    }
}
