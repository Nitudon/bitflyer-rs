use crate::api;
use crate::api::{ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use std::collections::HashMap;

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

pub async fn get_board(product_code: ProductCode) -> Result<BoardInfo, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
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
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_board_test() {
        let response = get_board(ProductCode::BTC_JPY).await;
        assert_eq!(response.is_ok(), true)
    }
}
