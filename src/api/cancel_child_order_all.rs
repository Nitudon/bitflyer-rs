use crate::api;
use crate::api::{ApiResponseError, ProductCode, BEFORE_QUERY_KEY, AFTER_QUERY_KEY, COUNT_QUERY_KEY, CurrencyCode, OrderStatusType, MESSAGE_ID_QUERY_KEY, ChildOrderType, OrderSideType};
use std::collections::HashMap;
use hyper::StatusCode;

const PATH : &'static str = "/v1/me/cancelallchildorder";

type CancelAllChildOrderRequest = CancelAllChildOrderInfo;

#[derive(Serialize, Debug)]
pub struct CancelAllChildOrderInfo {
    pub product_code: ProductCode,
}

pub async fn cancel_all_child_order(product_code: ProductCode) -> Result<StatusCode, ApiResponseError> {
    let request = CancelAllChildOrderInfo {
        product_code,
    };
    api::post::<CancelAllChildOrderRequest>(&PATH, &request).await
}
