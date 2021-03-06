use crate::api;
use crate::api::ApiResponseError;

const PATH : &'static str = "/v1/me/getcollateral";

type GetCollateralResponse = CollateralInfo;

#[derive(Deserialize, Debug)]
pub struct CollateralInfo {
    pub collateral: f32,
    pub open_position_pnl: f32,
    pub require_collateral: f32,
    pub keep_rate: f32
}

pub async fn get_collateral() -> Result<GetCollateralResponse, ApiResponseError> {
    api::get::<GetCollateralResponse>(&PATH).await
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
