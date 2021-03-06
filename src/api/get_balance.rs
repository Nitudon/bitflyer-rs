use crate::api;
use crate::api::ApiResponseError;

const PATH : &'static str = "/v1/me/getbalance";

type GetBalanceResponse = Vec<BalanceInfo>;

#[derive(Deserialize, Debug)]
pub struct BalanceInfo {
    pub currency_code: String,
    pub amount: f32,
    pub available: f32,
}

pub async fn get_balance() -> Result<GetBalanceResponse, ApiResponseError> {
    api::get::<GetBalanceResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_balance::get_balance;

    #[tokio::test]
    async fn get_balance_test() {
        test_api!(get_balance());
    }
}
