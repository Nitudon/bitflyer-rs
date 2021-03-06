use crate::api;
use crate::api::ApiResponseError;

const PATH : &'static str = "/v1/me/getpermissions";

type GetPermissionsResponse = Vec<String>;

pub async fn get_permissions() -> Result<GetPermissionsResponse, ApiResponseError> {
    api::get::<GetPermissionsResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_permissions::get_permissions;

    #[tokio::test]
    async fn get_permissions_test() {
        test_api!(get_permissions());
    }
}
