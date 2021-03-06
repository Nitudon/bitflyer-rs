use crate::api;
use crate::api::{HealthState, BoardState, ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY, CurrencyCode};
use std::str::FromStr;
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getcollateralaccounts";

type GetCollateralAccountsResponse = Vec<CollateralAccountInfo>;

#[derive(Deserialize, Debug)]
pub struct CollateralAccountInfo {
    pub currency_code: CurrencyCode,
    pub amount: f32,
}

pub async fn get_collateral_accounts() -> Result<GetCollateralAccountsResponse, ApiResponseError> {
    api::get::<GetCollateralAccountsResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_collateral_accounts::get_collateral_accounts;

    #[tokio::test]
    async fn get_collateral_accounts_test() {
        test_api!(get_collateral_accounts());
    }
}
