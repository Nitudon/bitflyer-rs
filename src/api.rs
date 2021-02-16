pub mod get_markets;
pub mod get_board;
pub mod get_ticker;
pub mod get_executions;
pub mod get_board_state;
pub mod get_health;
pub mod get_chats;

extern crate hyper;

use reqwest::{Url, RequestBuilder};
use hyper::header::{HeaderMap, CONTENT_TYPE};
use std::str::FromStr;
use std::collections::HashMap;
use std::fmt;

const ENDPOINT : &'static str = "https://api.bitflyer.com/v1/";
pub const PRODUCT_CODE_QUERY_KEY : &'static str = "product_code";
pub const MARKET_TYPE_QUERY_KEY : &'static str = "market_type";
pub const COUNT_QUERY_KEY : &'static str = "count";
pub const BEFORE_QUERY_KEY : &'static str = "before";
pub const AFTER_QUERY_KEY : &'static str = "after";

#[derive(Deserialize, Debug)]
pub enum HealthState {
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

#[derive(Deserialize, Debug)]
pub enum BoardState {
    RUNNING,
    CLOSED,
    STARTING,
    PROPEN,
    NOORDER,
    STOP
}

impl FromStr for BoardState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "RUNNING" => Ok(BoardState::RUNNING),
            "CLOSED" => Ok(BoardState::CLOSED),
            "STARTING" => Ok(BoardState::STARTING),
            "PROPEN" => Ok(BoardState::PROPEN),
            "NOORDER" => Ok(BoardState::NOORDER),
            "STOP" => Ok(BoardState::STOP),
            _ => Err(())
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum ProductCode {
    BTC_JPY,
    ETH_JPY,
    FX_BTC_JPY,
    ETH_BTC,
    BCH_BTC,
}

impl fmt::Display for ProductCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProductCode::BTC_JPY => write!(f, "[ProduceCode::BTC_JPY]"),
            ProductCode::ETH_JPY => write!(f, "[ProduceCode::ETH_JPY]"),
            ProductCode::FX_BTC_JPY => write!(f, "[ProduceCode::FX_BTC_JPY]"),
            ProductCode::ETH_BTC => write!(f, "[ProduceCode::ETH_BTC]"),
            ProductCode::BCH_BTC => write!(f, "[ProduceCode::BCH_BTC]"),
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

#[derive(Deserialize, Debug)]
pub enum MarketType {
    Spot,
    FX,
    Futures,
}

impl fmt::Display for MarketType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MarketType::Spot => write!(f, "[MarketType::Spot]"),
            MarketType::FX => write!(f, "[MarketType::FX]"),
            MarketType::Futures => write!(f, "[MarketType::Futures]"),
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

#[derive(Debug)]
pub enum ApiResponseError {
    Reqwest(reqwest::Error),
    UrlParse(url::ParseError),
    API(Vec<String>),
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
(endpoint: &str, query_map: &HashMap<String, String>) -> Result<T, reqwest::Error> {
    let url_str = http_url_with_params(endpoint, query_map).unwrap();
    get_impl(url_str).await
}

pub async fn get<T: serde::de::DeserializeOwned>
(endpoint: &str) -> Result<T, reqwest::Error> {
    let url_str = http_url(endpoint).unwrap();
    get_impl(url_str).await
}

async fn get_impl<T: serde::de::DeserializeOwned>
(endpoint: Url) -> Result<T, reqwest::Error> {
    reqwest::get(endpoint)
        .await?
        .json()
        .await
}

pub async fn post<T: serde::Serialize, U: serde::de::DeserializeOwned>
(endpoint: &str, body: &T) -> Result<U, reqwest::Error> {
    let url_str = http_url(endpoint).unwrap();
    post_impl(url_str, body).await
}

async fn post_impl<T: serde::Serialize, U: serde::de::DeserializeOwned>
(endpoint: Url, body: &T) -> Result<U, reqwest::Error> {
    reqwest::Client::new()
        .post(endpoint)
        .json(body)
        .send()
        .await?
        .json()
        .await
}

fn post_request<T: serde::Serialize, U: serde::de::DeserializeOwned>
(method: &str, body: &T) -> RequestBuilder {
    let url_str = http_url(method).unwrap();
    reqwest::Client::new()
        .post(url_str)
        .json(body)
}

fn http_url_with_params(method: &str, query_map: &HashMap<String, String>) -> Result<Url, url::ParseError> {
    let url_str = format!("{}{}", ENDPOINT, method);
    Url::parse_with_params(&url_str, query_map)
}

fn http_url(method: &str) -> Result<Url, url::ParseError> {
    let url_str = format!("{}{}", ENDPOINT, method);
    Url::parse(&url_str)
}

fn http_header() -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    header
}
