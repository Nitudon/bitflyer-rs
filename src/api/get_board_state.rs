use crate::api;
use crate::api::{HealthState, BoardState, ApiResponseError};
use std::str::FromStr;

const METHOD : &'static str = "getboardstate";

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

pub async fn get_board_state() -> Result<BoardStateInfo, ApiResponseError> {
    let response = api::get::<GetBoardStateResponse>(&METHOD).await?;

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
    use crate::api::get_board_state::get_board_state;

    #[tokio::test]
    async fn get_board_state_test() {
        let response = get_board_state().await;
        assert_eq!(response.is_ok(), true)
    }
}
