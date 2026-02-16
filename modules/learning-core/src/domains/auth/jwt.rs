use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};

const TOKEN_EXPIRY_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub member_id: Uuid,
    pub is_admin: bool,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
    pub jti: String,
}

pub fn create_token(
    member_id: Uuid,
    is_admin: bool,
    secret: &str,
    issuer: &str,
) -> Result<String> {
    let now = Utc::now().timestamp();
    let claims = Claims {
        sub: member_id.to_string(),
        member_id,
        is_admin,
        exp: now + (TOKEN_EXPIRY_HOURS * 3600),
        iat: now,
        iss: issuer.to_string(),
        jti: Uuid::new_v4().to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(Error::Jwt)
}

pub fn validate_token(token: &str, secret: &str, issuer: &str) -> Result<Claims> {
    let mut validation = Validation::default();
    validation.set_issuer(&[issuer]);

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
    .map(|data| data.claims)
    .map_err(Error::Jwt)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_validate_token() {
        let member_id = Uuid::new_v4();
        let secret = "test-secret";
        let issuer = "ourmeadow";

        let token = create_token(member_id, false, secret, issuer).unwrap();
        let claims = validate_token(&token, secret, issuer).unwrap();

        assert_eq!(claims.member_id, member_id);
        assert!(!claims.is_admin);
        assert_eq!(claims.iss, issuer);
    }

    #[test]
    fn test_invalid_secret_fails() {
        let member_id = Uuid::new_v4();
        let token = create_token(member_id, false, "secret1", "ourmeadow").unwrap();
        let result = validate_token(&token, "wrong-secret", "ourmeadow");
        assert!(result.is_err());
    }
}
