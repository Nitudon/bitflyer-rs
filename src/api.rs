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

const ENDPOINT : &'static str = "https://api.bitflyer.com/v1/";

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

pub async fn get<T: serde::de::DeserializeOwned>
(endpoint: &str) -> Result<T, reqwest::Error> {
    let url_str = http_url(endpoint).unwrap();
    reqwest::get(url_str)
        .await?
        .json()
        .await
}

pub async fn post<T: serde::Serialize, U: serde::de::DeserializeOwned>
(endpoint: &str, body: &T) -> Result<U, reqwest::Error> {
    let url_str = http_url(endpoint).unwrap();
    reqwest::Client::new()
        .post(url_str)
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

fn http_url(method: &str) -> Result<Url, url::ParseError> {
    let url_str = format!("{}{}", ENDPOINT, method);
    Url::parse(&url_str)
}

fn http_header() -> HeaderMap {
    let mut header = HeaderMap::new();
    header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    header
}
