use crate::api;
use crate::api::{ApiResponseError, FROM_DATE_QUERY_KEY};
use chrono::NaiveDateTime;
use std::collections::HashMap;

const PATH : &'static str = "/v1/getchats";

type GetChatsResponse = Vec<ChatInfo>;

#[derive(Deserialize, Debug)]
pub struct ChatInfo {
    pub nickname: String,
    pub message: String,
    pub date: NaiveDateTime,
}

pub async fn get_chats(from_date: &i64) -> Result<GetChatsResponse, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(FROM_DATE_QUERY_KEY.to_string(), from_date.to_string());
    api::get::<GetChatsResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_chats::get_chats;
    use chrono::{Utc, Duration};

    #[tokio::test]
    async fn get_chats_test() {
        let time = Utc::now() - Duration::minutes(3);
        let timestamp = time.timestamp();
        test_api!(get_chats(&timestamp));
    }
}
