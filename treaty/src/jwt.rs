use chrono::{DateTime, Duration, Utc};

use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha384;
use std::collections::BTreeMap;

pub fn create_jwt(host_name: &str, login: &str) -> (String, DateTime<Utc>) {
    // this duration should be a config item
    let expiration = Utc::now() + Duration::minutes(20);
    let exp_string = expiration.to_rfc3339();

    // this secret should be stored in a config file: "treaty_item"
    let key: Hmac<Sha384> = Hmac::new_from_slice(b"treaty_item").unwrap();
    let header = Header {
        algorithm: AlgorithmType::Hs384,
        ..Default::default()
    };

    let mut claims = BTreeMap::new();

    claims.insert("sub", login);
    claims.insert("iss", host_name);
    claims.insert("exp", &exp_string);

    let token = Token::new(header, claims).sign_with_key(&key).unwrap();
    let token_str = token.as_str();

    (token_str.to_string(), expiration)
}
