use crate::api;
use crate::api::{HealthState, ApiResponseError};
use std::str::FromStr;

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

pub async fn get_health() -> Result<StateInfo, ApiResponseError> {
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

    #[tokio::test]
    async fn get_health_test() {
        let health = get_health().await;
        assert_eq!(health.is_ok(), true)
    }
}
