use crate::api;
use crate::api::{HealthState, ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use std::str::FromStr;
use std::collections::HashMap;

const METHOD : &'static str = "gethealth";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetHealthResponse {
    Error { errors: Vec<String> },
    Response { status: String }
}

#[derive(Deserialize, Debug)]
pub struct StateInfo {
    pub status: HealthState,
}

pub async fn get_health(product_code: ProductCode) -> Result<StateInfo, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    let response = api::get::<GetHealthResponse>(&METHOD).await?;

    match response {
        GetHealthResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetHealthResponse::Response { status } => Ok(StateInfo{
            status: HealthState::from_str(&status).unwrap()}),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_health::get_health;
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_health_test() {
        let health = get_health(ProductCode::BTC_JPY).await;
        assert_eq!(health.is_ok(), true)
    }
}
