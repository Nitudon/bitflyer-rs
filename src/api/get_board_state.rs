use crate::api;
use crate::api::{HealthState, BoardState, ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use std::str::FromStr;
use std::collections::HashMap;

const PATH : &'static str = "/v1/getboardstate";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetBoardStateResponse {
    Error { errors: Vec<String> },
    Response { health: String, state: String, data: Option<OptionalBoardData> }
}

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

pub async fn get_board_state(product_code: ProductCode) -> Result<BoardStateInfo, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    let response = api::get_with_params::<GetBoardStateResponse>(&PATH, &params).await?;

    match response {
        GetBoardStateResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetBoardStateResponse::Response { health, state, data } => Ok(BoardStateInfo{
            health: HealthState::from_str(&health).unwrap(), 
            state: BoardState::from_str(&state).unwrap(), 
            data}),
    }
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
