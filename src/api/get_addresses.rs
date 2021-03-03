use crate::api;
use crate::api::{HealthState, BoardState, ApiResponseError, ProductCode, CurrencyCode, AddressType};
use std::str::FromStr;
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/getaddresses";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetAddressesResponse {
    Error { errors: Vec<String> },
    Response(Vec<AddressInfo>)
}

#[derive(Deserialize, Debug)]
pub struct AddressInfo {
    pub r#type: AddressType,
    pub currency_code: CurrencyCode,
    pub address: String,
}

pub async fn get_addresses() -> Result<Vec<AddressInfo>, ApiResponseError> {
    let response = api::get::<GetAddressesResponse>(&PATH).await?;

    match response {
        GetAddressesResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetAddressesResponse::Response (vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_addresses::get_addresses;

    #[tokio::test]
    async fn get_addresses_test() {
        test_api!(get_addresses());
    }
}
