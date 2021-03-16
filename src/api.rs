pub mod get_markets;
pub mod get_board;
pub mod get_ticker;
pub mod get_executions;
pub mod get_board_state;
pub mod get_health;
pub mod get_chats;
pub mod get_permissions;
pub mod get_balance;
pub mod get_collateral;
pub mod get_collateral_accounts;
pub mod get_addresses;
pub mod get_coinins;
pub mod get_coinouts;
pub mod get_bank_accounts;
pub mod get_deposits;
pub mod get_withdrawals;
pub mod send_child_order;

extern crate hyper;

use reqwest::{Url, Response, StatusCode};
use hyper::header::{HeaderMap, CONTENT_TYPE, HeaderName};
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

use crate::auth::credential::*;
use hyper::http::HeaderValue;
use hyper::Method;

const ENDPOINT : &'static str = "https://api.bitflyer.com";
pub const FROM_DATE_QUERY_KEY : &'static str = "from_date";
pub const PRODUCT_CODE_QUERY_KEY : &'static str = "product_code";
pub const MARKET_TYPE_QUERY_KEY : &'static str = "market_type";
pub const COUNT_QUERY_KEY : &'static str = "count";
pub const BEFORE_QUERY_KEY : &'static str = "before";
pub const AFTER_QUERY_KEY : &'static str = "after";
pub const MESSAGE_ID_QUERY_KEY : &'static str = "message_id";

#[derive(Serialize, Deserialize, Debug)]
pub enum HealthState {
    Unknown,
    NORMAL,
    BUSY,
    VERYBUSY,
    SUPERBUSY,
    NOORDER,
    STOP
}

impl FromStr for HealthState {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "NORMAL" => Ok(HealthState::NORMAL),
            "BUSY" => Ok(HealthState::BUSY),
            "VERYBUSY" => Ok(HealthState::VERYBUSY),
            "SUPERBUSY" => Ok(HealthState::SUPERBUSY),
            "NOORDER" => Ok(HealthState::NOORDER),
            "STOP" => Ok(HealthState::STOP),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BoardState {
    Unknown,
    RUNNING,
    CLOSED,
    STARTING,
    PREOPEN,
    CIRCUITBREAK,
    AWAITINGSQ,
    MATURED
}

impl FromStr for BoardState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "RUNNING" => Ok(BoardState::RUNNING),
            "CLOSED" => Ok(BoardState::CLOSED),
            "STARTING" => Ok(BoardState::STARTING),
            "PREOPEN" => Ok(BoardState::PREOPEN),
            "CIRCUITBREAK" => Ok(BoardState::CIRCUITBREAK),
            "AWAITINGSQ" => Ok(BoardState::AWAITINGSQ),
            "MATURED" => Ok(BoardState::MATURED),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ProductCode {
    Unknown,
    BTC_JPY,
    ETH_JPY,
    FX_BTC_JPY,
    ETH_BTC,
    BCH_BTC,
}

impl fmt::Display for ProductCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProductCode::BTC_JPY => write!(f, "BTC_JPY"),
            ProductCode::ETH_JPY => write!(f, "ETH_JPY"),
            ProductCode::FX_BTC_JPY => write!(f, "FX_BTC_JPY"),
            ProductCode::ETH_BTC => write!(f, "ETH_BTC"),
            ProductCode::BCH_BTC => write!(f, "BCH_BTC"),
            _ => write!(f, "Unknown")
        }
    }
}

impl FromStr for ProductCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "BTC_JPY" => Ok(ProductCode::BTC_JPY),
            "ETH_JPY" => Ok(ProductCode::ETH_JPY),
            "FX_BTC_JPY" => Ok(ProductCode::FX_BTC_JPY),
            "ETH_BTC" => Ok(ProductCode::ETH_BTC),
            "BCH_BTC" => Ok(ProductCode::BCH_BTC),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CurrencyCode {
    Unknown,
    JPY,
    BTC,
    ETH,
    BCH
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CurrencyCode::JPY => write!(f, "JPY"),
            CurrencyCode::BTC => write!(f, "BTC"),
            CurrencyCode::ETH => write!(f, "ETH"),
            CurrencyCode::BCH => write!(f, "BCH"),
            _ => write!(f, "Unknown")
        }
    }
}

impl FromStr for CurrencyCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "JPY" => Ok(CurrencyCode::JPY),
            "BTC" => Ok(CurrencyCode::BTC),
            "ETH" => Ok(CurrencyCode::ETH),
            "BCH" => Ok(CurrencyCode::BCH),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MarketType {
    Spot,
    FX,
    Futures,
}

impl fmt::Display for MarketType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MarketType::Spot => write!(f, "Spot"),
            MarketType::FX => write!(f, "FX"),
            MarketType::Futures => write!(f, "Futures"),
        }
    }
}

impl FromStr for MarketType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Spot" => Ok(MarketType::Spot),
            "FX" => Ok(MarketType::FX),
            "Futures" => Ok(MarketType::Futures),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AddressType {
    Unknown,
    NORMAL,
}

impl fmt::Display for AddressType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressType::NORMAL => write!(f, "NORMAL"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl FromStr for AddressType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "NORMAL" => Ok(AddressType::NORMAL),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderStatusType {
    Unknown,
    PENDING,
    COMPLETED
}

impl fmt::Display for OrderStatusType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderStatusType::PENDING => write!(f, "PENDING"),
            OrderStatusType::COMPLETED => write!(f, "COMPLETED"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl FromStr for OrderStatusType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "PENDING" => Ok(OrderStatusType::PENDING),
            "COMPLETED" => Ok(OrderStatusType::COMPLETED),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChildOrderType {
    Unknown,
    LIMIT,
    MARKET
}

impl fmt::Display for ChildOrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChildOrderType::LIMIT => write!(f, "LIMIT"),
            ChildOrderType::MARKET => write!(f, "MARKET"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl FromStr for ChildOrderType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "LIMIT" => Ok(ChildOrderType::LIMIT),
            "MARKET" => Ok(ChildOrderType::MARKET),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum OrderSideType {
    Unknown,
    BUY,
    SELL
}

impl fmt::Display for OrderSideType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderSideType::BUY => write!(f, "BUY"),
            OrderSideType::SELL => write!(f, "SELL"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl FromStr for OrderSideType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "BUY" => Ok(OrderSideType::BUY),
            "SELL" => Ok(OrderSideType::SELL),
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum ApiResponseError {
    Reqwest(reqwest::Error),
    UrlParse(url::ParseError),
    API(StatusCode, Vec<String>),
    ResponseParse(Vec<String>)
}

impl From<Vec<String>> for ApiResponseError {
    fn from(e: Vec<String>) -> ApiResponseError {
        ApiResponseError::ResponseParse(e)
    }
}

impl From<(StatusCode, Vec<String>)> for ApiResponseError {
    fn from(e: (StatusCode, Vec<String>)) -> ApiResponseError {
        ApiResponseError::API(e.0, e.1)
    }
}

impl From<reqwest::Error> for ApiResponseError {
    fn from(e: reqwest::Error) -> ApiResponseError {
        ApiResponseError::Reqwest(e)
    }
}

impl From<url::ParseError> for ApiResponseError {
    fn from(e: url::ParseError) -> ApiResponseError {
        ApiResponseError::UrlParse(e)
    }
}

pub async fn get_with_params<T: serde::de::DeserializeOwned>
(path: &str, query_map: &HashMap<String, String>) -> Result<T, ApiResponseError> {
    let header = http_header(&Method::GET.to_string(), path, "").unwrap();
    let url = http_url_with_params(path, query_map)?;
    let get = get_impl(url, header).await;
    match get {
        Ok(t) => {
            if t.status().is_success() {
                Ok(t.json().await?)
            } else {
                Err(ApiResponseError::from((t.status(), t.json().await?)))
            }
        },
        Err(e) => Err(ApiResponseError::from(e))
    }
}

pub async fn get<T: serde::de::DeserializeOwned>
(path: &str) -> Result<T, ApiResponseError> {
    let header = http_header(&Method::GET.to_string(), path, "").unwrap();
    let url = http_url(path)?;
    let get = get_impl(url, header).await;
    match get {
        Ok(t) => {
            if t.status().is_success() {
                Ok(t.json().await?)
            } else {
                Err(ApiResponseError::from((t.status(), t.json().await?)))
            }
        },
        Err(e) => Err(ApiResponseError::from(e))
    }
}

async fn get_impl(url: Url, header: HeaderMap) -> Result<Response, reqwest::Error> {
    reqwest::Client::new()
        .get(url)
        .headers(header)
        .send()
        .await
}

pub async fn post<T: serde::Serialize, U: serde::de::DeserializeOwned>
(path: &str, body: &T) -> Result<U, ApiResponseError> {
    let url = http_url(path)?;
    let body_json = serde_json::to_string(body).unwrap();
    let header = http_header(&Method::POST.to_string(), path, &body_json).unwrap();
    let post = post_impl(url, header, body).await;
    match post {
        Ok(t) => {
            if t.status().is_success() {
                Ok(t.json().await?)
            } else {
                Err(ApiResponseError::from((t.status(), t.json().await?)))
            }
        },
        Err(e) => { Err(ApiResponseError::from(e)) }
    }
}

async fn post_impl<T: serde::Serialize>
(url: Url, header: HeaderMap, body: &T) -> Result<Response, reqwest::Error> {
    reqwest::Client::new()
        .post(url)
        .headers(header)
        .json(body)
        .send()
        .await
}

fn http_url_with_params(path: &str, query_map: &HashMap<String, String>) -> Result<Url, url::ParseError> {
    let url_str = format!("{}{}", ENDPOINT, path);
    Url::parse_with_params(&url_str, query_map)
}

fn http_url(path: &str) -> Result<Url, url::ParseError> {
    let url_str = format!("{}{}", ENDPOINT, path);
    Url::parse(&url_str)
}

fn http_header(method: &str, path: &str, body: &str) -> Result<HeaderMap, CredentialError> {
    let mut header = HeaderMap::new();
    let credential = get_credential(&method.to_string(), &path.to_string(), &body.to_string());
    if credential.is_err() {
        return Err(credential.err().unwrap())
    }
    
    let content_type = "application/json".parse();
    header.insert(CONTENT_TYPE, content_type.unwrap());
    for (k, v) in credential.unwrap() {
        let header_name = k.parse::<HeaderName>();
        let header_value = v.parse::<HeaderValue>();
        header.insert(header_name.unwrap(), header_value.unwrap());
    }
    
    Ok(header)
}

#[macro_export]
macro_rules! test_api {
    ($val: expr) => {
        let response = $val.await;
        if response.is_err() {
            println!("{:?}", response);
        }
        assert_eq!(response.is_ok(), true)
    };
}
