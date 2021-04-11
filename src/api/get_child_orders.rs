use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, OrderStatusType, OrderSideType, ChildOrderType, ChildOrderState};
use std::collections::HashMap;
use chrono::NaiveDateTime;

const PATH : &'static str = "/v1/me/getchildorders";

type GetChildOrdersResponse = Vec<ChildOrderInfo>;

#[derive(Deserialize, Debug)]
pub struct ChildOrderInfo {
    pub id: u32,
    pub child_order_id: String,
    pub product_code: ProductCode,
    pub side: OrderSideType,
    pub child_order_type: ChildOrderType,
    pub price: f32,
    pub average_price: f32,
    pub size: f32,
    pub child_order_state: ChildOrderState,
    pub expire_date: NaiveDateTime,
    pub child_order_date: NaiveDateTime,
    pub child_order_acceptance_id: String,
    pub outstanding_size: usize,
    pub cancel_size: usize,
    pub executed_size: usize,
    pub total_commission: usize
}

pub async fn get_child_orders(before: u32, after: u32, count: i32) -> Result<GetChildOrdersResponse, ApiResponseError> {
    let mut params = HashMap::new();
    params.insert(BEFORE_QUERY_KEY.to_string(), before.to_string());
    params.insert(AFTER_QUERY_KEY.to_string(), after.to_string());
    params.insert(COUNT_QUERY_KEY.to_string(), count.to_string());
    api::get::<GetChildOrdersResponse>(&PATH).await
}

#[cfg(test)]
mod tests {
    use crate::test_api;
    use crate::api::get_child_orders::get_child_orders;

    #[tokio::test]
    async fn get_child_orders_test() {
        test_api!(get_child_orders(999, 0, 10));
    }
}
