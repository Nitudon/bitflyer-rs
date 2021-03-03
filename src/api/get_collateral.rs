use crate::api;
use crate::api::ApiResponseError;

const PATH : &'static str = "/v1/me/getcollateral";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetCollateralResponse {
    Error { errors: Vec<String> },
    Response { collateral: f32, open_position_pnl: f32, require_collateral: f32, keep_rate: f32}
}

#[derive(Deserialize, Debug)]
pub struct CollateralInfo {
    pub collateral: f32,
    pub open_position_pnl: f32,
    pub require_collateral: f32,
    pub keep_rate: f32
}

pub async fn get_collateral() -> Result<CollateralInfo, ApiResponseError> {
    let response = api::get::<GetCollateralResponse>(&PATH).await?;

    match response {
        GetCollateralResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetCollateralResponse::Response {collateral, open_position_pnl, require_collateral, keep_rate} => Ok(CollateralInfo{
            collateral,
            open_position_pnl,
            require_collateral,
            keep_rate
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_collateral::get_collateral;

    #[tokio::test]
    async fn get_collateral_test() {
        test_api!(get_collateral());
    }
}
