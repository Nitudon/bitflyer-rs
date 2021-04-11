use crate::api;
use crate::api::{HealthState, ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use std::str::FromStr;
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/gettradingcommission";

type GetTradingCommissionResponse = TradingCommissionInfo;

#[derive(Deserialize, Debug)]
pub struct TradingCommissionInfo {
    pub commission_rate: f32,
}

pub async fn get_trading_commission(product_code: ProductCode) -> Result<GetTradingCommissionResponse, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    api::get::<GetTradingCommissionResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_trading_commission::get_trading_commission;
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_trading_commission_test() {
        test_api!(get_trading_commission(ProductCode::BTC_JPY));
    }
}
