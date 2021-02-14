use crate::api;
use crate::api::ApiResponseError;

const METHOD : &'static str = "markets";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetMarketResponse {
    Error { errors: Vec<String> },
    Response(Vec<MarketInfo>)
}

#[derive(Deserialize, Debug)]
pub struct MarketInfo {
    pub product_code: String,
    pub market_type: String,
    pub alias: Option<String>,
}

pub async fn get_markets() -> Result<Vec<MarketInfo>, ApiResponseError> {
    let response = api::get::<GetMarketResponse>(&METHOD).await?;
    
    match response {
        GetMarketResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetMarketResponse::Response(vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_markets::get_markets;

    #[tokio::test]
    async fn get_markets_test() {
        let markets = get_markets().await;
        assert_eq!(markets.is_ok(), true)
    }
}
