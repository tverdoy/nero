use crate::error::{NeroError, NeroErrorKind, NeroResult};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_token<T: ToString>(exp: u32, sub: T, secret_key: &[u8]) -> NeroResult<String> {
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(exp.into()))
        .ok_or(NeroError::new_simple(NeroErrorKind::GenerateToken))?
        .timestamp() as usize;

    let claims = Claims {
        sub: sub.to_string(),
        exp,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key),
    )
    .map_err(|e| NeroError::new(NeroErrorKind::GenerateToken, e))
}

pub fn verify_token<T: ToString>(token: T, secret_key: &[u8]) -> NeroResult<String> {
    let claims: Claims = decode(
        &token.to_string(),
        &DecodingKey::from_secret(secret_key),
        &Validation::default(),
    )
    .map_err(|e| NeroError::new(NeroErrorKind::VerifyToken, e))?
    .claims;

    Ok(claims.sub)
}
