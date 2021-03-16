use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, OrderStatusType, MESSAGE_ID_QUERY_KEY, ChildOrderType, OrderSideType};
use std::collections::HashMap;

const PATH : &'static str = "/v1/me/sendchildorder";

type SendChildOrderRequest = ChildOrderInfo;
type SendChildOrderResponse = ChildOrderAcceptance;

#[derive(Serialize, Debug)]
pub struct ChildOrderInfo {
    pub product_code: ProductCode,
    pub child_order_type: ChildOrderType,
    pub side: OrderSideType,
    pub price: Option<f32>,
    pub size: f32,
    pub minutes_to_expire: i32,
}

#[derive(Deserialize, Debug)]
pub struct ChildOrderAcceptance {
    pub child_order_acceptance_id: String,
}

pub async fn send_child_order(product_code: ProductCode, child_order_type: ChildOrderType, side: OrderSideType, price: Option<f32>, size: f32, minutes_to_expire: i32) -> Result<SendChildOrderResponse, ApiResponseError> {
    let request = ChildOrderInfo {
        product_code,
        child_order_type,
        side,
        price,
        size,
        minutes_to_expire
    };
    api::post_with_response::<SendChildOrderRequest, SendChildOrderResponse>(&PATH, &request).await
}
