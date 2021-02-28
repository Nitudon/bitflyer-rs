use crate::api;
use crate::api::ApiResponseError;

const PATH : &'static str = "/v1/me/getbalance";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetBalanceResponse {
    Error { errors: Vec<String> },
    Response(Vec<BalanceInfo>)
}

#[derive(Deserialize, Debug)]
pub struct BalanceInfo {
    pub currency_code: String,
    pub amount: f32,
    pub available: f32,
}

pub async fn get_balance() -> Result<Vec<BalanceInfo>, ApiResponseError> {
    let response = api::get::<GetBalanceResponse>(&PATH).await?;

    match response {
        GetBalanceResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetBalanceResponse::Response(vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_balance::get_balance;

    #[tokio::test]
    async fn get_balance_test() {
        let balance = get_balance().await;
        assert_eq!(balance.is_ok(), true)
    }
}
