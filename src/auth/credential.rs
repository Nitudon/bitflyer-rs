use std::env;
use hmac::{NewMac, Hmac, Mac};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::string::String;
use sha2::Sha256;
use itertools::Itertools;

const API_KEY : &'static str = "BF_API_KEY";
const API_SECRET : &'static str = "BF_API_SECRET";

#[derive(Debug)]
pub enum CredentialError {
    Var(env::VarError),
    SignParse(std::string::FromUtf8Error),
}

pub fn get_credential(method: &String, path: &String, body: &String) -> Result<HashMap<String, String>, CredentialError> {
    let utc: DateTime<Utc>= Utc::now();
    let timestamp = utc.timestamp().to_string();
    
    let api_key = get_access_key()
        .or_else(|e| return Err(e));
    let api_secret = get_access_secret()
        .or_else(|e| return Err(e));
    let sign = get_access_sign(method, path, &body, &timestamp, &api_secret?);

    let utc: DateTime<Utc>= Utc::now();
    let timestamp = utc.timestamp().to_string();
    let mut map = HashMap::new();
    map.insert("ACCESS-KEY".to_string(), api_key?);
    map.insert("ACCESS-TIMESTAMP".to_string(), timestamp);
    map.insert("ACCESS-SIGN".to_string(), sign);
    Ok(map)
}

fn get_access_sign(method: &String, path: &String, body: &String, timestamp: &String, secret: &String) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let text = [timestamp.clone(), method.clone(), path.clone(), body.clone()].concat();
    let update = text.as_bytes();
    let secret_bytes = secret.as_bytes();
    let mut mac = HmacSha256::new_varkey(secret_bytes).expect("Invalid Key");
    mac.update(update);
    let output = mac.finalize();
    let output_bytes = output.into_bytes();
    output_bytes.iter().format_with("", |byte, f| f(&format_args!("{:02x}", byte))).to_string()
}

fn get_access_key() -> Result<String, CredentialError> {
    get_env_var(API_KEY)
}

fn get_access_secret() -> Result<String, CredentialError> {
    get_env_var(API_SECRET)
}

fn get_env_var(key: &str) -> Result<String, CredentialError> {
    let var = env::var(key);
    match var {
        Ok(var) => Ok(var),
        Err(err) => Err(CredentialError::Var(err))
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Method;
    use crate::api;
    use std::str;
    use chrono::DateTime;
    use crate::auth::credential::*;
    use itertools::Itertools;

    #[tokio::test]
    async fn test_credential() {
        let method = Method::GET.to_string();
        let path = "/v1/me/getbalance".to_string();
        let body = String::new();
        let utc: DateTime<Utc>= Utc::now();
        let timestamp = utc.timestamp().to_string();

        let credential = get_credential(&method, &path, &body);
        assert_eq!(credential.is_ok(), true)
    }
}