use crate::api;
use crate::api::{HealthState, BoardState, ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY, CurrencyCode};
use std::str::FromStr;
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getcollateralaccounts";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetCollateralAccountsResponse {
    Error { errors: Vec<String> },
    Response(Vec<CollateralAccountInfo>)
}

#[derive(Deserialize, Debug)]
pub struct CollateralAccountInfo {
    pub currency_code: CurrencyCode,
    pub amount: f32,
}

pub async fn get_collateral_accounts() -> Result<Vec<CollateralAccountInfo>, ApiResponseError> {
    let response = api::get::<GetCollateralAccountsResponse>(&PATH).await?;

    match response {
        GetCollateralAccountsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetCollateralAccountsResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_collateral_accounts::get_collateral_accounts;

    #[tokio::test]
    async fn get_collateral_accounts_test() {
        let response = get_collateral_accounts().await;
        assert_eq!(response.is_ok(), true)
    }
}
