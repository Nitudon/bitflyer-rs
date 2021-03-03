use crate::api;
use crate::api::ApiResponseError;
use chrono::NaiveDateTime;

const PATH : &'static str = "/v1/getchats";

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum GetChatsResponse {
    Error { errors: Vec<String> },
    Response(Vec<ChatInfo>)
}

#[derive(Deserialize, Debug)]
pub struct ChatInfo {
    pub nickname: String,
    pub message: String,
    pub date: NaiveDateTime,
}

pub async fn get_chats() -> Result<Vec<ChatInfo>, ApiResponseError> {
    let response = api::get::<GetChatsResponse>(&PATH).await?;

    match response {
        GetChatsResponse::Error { errors } => Err(ApiResponseError::API(errors)),
        GetChatsResponse::Response(vec) => Ok(vec),
    }
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_chats::get_chats;

    #[tokio::test]
    async fn get_chats_test() {
        test_api!(get_chats());
    }
}
