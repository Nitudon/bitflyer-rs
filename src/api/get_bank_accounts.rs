use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, CoinInStatusType};
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getcoinins";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetBankAccountsResponse {
    Error { errors: Vec<String> },
    Response (Vec<BankAccountInfo>)
}

#[derive(Deserialize, Debug)]
pub struct BankAccountInfo {
    pub id: u32,
    pub is_verified: bool,
    pub bank_name: String,
    pub branch_name: String,
    pub account_type: String,
    pub account_name: String,
    pub account_number: String,
}

pub async fn get_bank_accounts() -> Result<Vec<BankAccountInfo>, ApiResponseError> {
    let response = api::get::<GetBankAccountsResponse>(&PATH).await?;

    match response {
        GetBankAccountsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetBankAccountsResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_bank_accounts::get_bank_accounts;

    #[tokio::test]
    async fn get_bank_accounts_test() {
        test_api!(get_bank_accounts());
    }
}
