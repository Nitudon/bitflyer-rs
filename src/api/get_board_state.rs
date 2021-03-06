use crate::api;
use crate::api::{HealthState, BoardState, ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use std::str::FromStr;
use std::collections::HashMap;

const PATH : &'static str = "/v1/getboardstate";

type GetBoardStateResponse = BoardStateInfo;

#[derive(Deserialize, Debug)]
pub struct BoardStateInfo {
    pub health: HealthState,
    pub state: BoardState,
    pub data: Option<OptionalBoardData>
}

#[derive(Deserialize, Debug)]
pub struct OptionalBoardData {
    pub special_quotation: i32
}

pub async fn get_board_state(product_code: ProductCode) -> Result<GetBoardStateResponse, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    api::get_with_params::<GetBoardStateResponse>(&PATH, &params).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_board_state::get_board_state;
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_board_state_test() {
        test_api!(get_board_state(ProductCode::BTC_JPY));
    }
}
