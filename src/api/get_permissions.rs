use crate::api;
use crate::api::ApiResponseError;

const PATH : &'static str = "/v1/me/getpermissions";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetPermissionsResponse {
    Error { errors: Vec<String> },
    Response(Vec<String>)
}

pub async fn get_permissions() -> Result<Vec<String>, ApiResponseError> {
    let response = api::get::<GetPermissionsResponse>(&PATH).await?;

    match response {
        GetPermissionsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetPermissionsResponse::Response(vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::get_permissions::get_permissions;

    #[tokio::test]
    async fn get_permissions_test() {
        let balance = get_permissions().await;
        assert_eq!(balance.is_ok(), true)
    }
}
