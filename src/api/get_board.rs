use crate::api;
use crate::api::ApiResponseError;

const METHOD : &'static str = "board";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetBoardResponse {
    Error { errors: Vec<String> },
    Response { mid_price: f32, bids: Vec<BoardBid>, asks: Vec<BoardAsk> }
}

#[derive(Deserialize, Debug)]
pub struct BoardInfo {
    pub mid_price: f32,
    pub bids : Vec<BoardBid>,
    pub asks : Vec<BoardAsk>
}

#[derive(Deserialize, Debug)]
pub struct BoardBid {
    pub price: f32,
    pub size: f32,
}

#[derive(Deserialize, Debug)]
pub struct BoardAsk {
    pub price: f32,
    pub size: f32,
}

pub async fn get_board() -> Result<BoardInfo, ApiResponseError> {
    let response = api::get::<GetBoardResponse>(&METHOD).await?;

    match response {
        GetBoardResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetBoardResponse::Response { mid_price, bids, asks} => Ok(BoardInfo{
            mid_price,
            bids,
            asks
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_board::get_board;

    #[tokio::test]
    async fn get_board_test() {
        let response = get_board().await;
        assert_eq!(response.is_ok(), true)
    }
}
