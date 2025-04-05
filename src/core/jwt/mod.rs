use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// TODO
/// This init has to be done better
/// Using cfg so that i won't compile for release
#[cfg(debug_assertions)]
pub const SECRET: &[u8; 38] = b"Salut, c'est un secret de developement";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub exp: usize, // expiration date
    pub iat: usize, // issued at
    pub id: i64,    // User ID the token refers to
}

pub fn encode_token(user_id: i64, secret: &[u8]) -> Result<String, jsonwebtoken::errors::Error> {
    // Fill the claims values
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::hours(24)).timestamp() as usize; // Token valid for 24 hours

    let claims = TokenClaims {
        exp,
        iat,
        id: user_id,
    };

    // Default encoding
    let header = Header::default();
    let encoding_key = EncodingKey::from_secret(secret);

    encode(&header, &claims, &encoding_key)
}

pub fn decode_token(
    token: &str,
    secret: &[u8],
) -> Result<TokenClaims, jsonwebtoken::errors::Error> {
    // Get the decoding key
    let decoding_key = DecodingKey::from_secret(secret);

    // Decode the token with validation
    let validation = Validation::default();
    let token_data = decode::<TokenClaims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
