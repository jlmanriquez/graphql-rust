use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

const SECRET_KEY: &[u8] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_token(username: &str) -> Result<String, String> {
    let expiration: usize = (chrono::Utc::now() + Duration::hours(24))
        .timestamp()
        .try_into()
        .unwrap();
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration,
    };
    let token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    ) {
        Ok(token) => token,
        Err(error) => return Err(error.to_string()),
    };

    Ok(token)
}

pub fn parse_token(token_str: &str) -> Result<String, String> {
    let token = decode::<Claims>(
        token_str,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    );

    match token {
        Ok(data) => Ok(data.claims.sub),
        Err(error) => Err(error.to_string()),
    }
}
