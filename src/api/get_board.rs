use crate::api;
use crate::api::{ApiResponseError, ProductCode, PRODUCT_CODE_QUERY_KEY};
use std::collections::HashMap;

const PATH : &'static str = "/v1/board";

type GetBoardResponse = BoardInfo;

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

pub async fn get_board(product_code: ProductCode) -> Result<GetBoardResponse, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(PRODUCT_CODE_QUERY_KEY.to_string(), product_code.to_string());
    api::get::<GetBoardResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_board::get_board;
    use crate::api::ProductCode;

    #[tokio::test]
    async fn get_board_test() {
        test_api!(get_board(ProductCode::BTC_JPY));
    }
}
