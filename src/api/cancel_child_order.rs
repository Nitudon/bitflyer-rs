use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, OrderStatusType, MESSAGE_ID_QUERY_KEY, ChildOrderType, OrderSideType};
use std::collections::HashMap;
use hyper::StatusCode;

const PATH : &'static str = "/v1/me/cancelchildorder";

type CancelChildOrderRequest = CancelChildOrderInfo;

#[derive(Serialize, Debug)]
pub struct CancelChildOrderInfo {
    pub product_code: ProductCode,
    pub child_order_id: String,
}

pub async fn cancel_child_order(product_code: ProductCode, child_order_id: String) -> Result<StatusCode, ApiResponseError> {
    let request = CancelChildOrderInfo {
        product_code,
        child_order_id,
    };
    api::post::<CancelChildOrderRequest>(&PATH, &request).await
}
