use sha2::{Digest, Sha256};
use sqlx::PgPool;

use super::models::{Identifier, Member};
use crate::error::Result;

pub fn hash_phone_number(phone: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(phone.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn is_admin_identifier(phone: &str, admin_identifiers: &str) -> bool {
    admin_identifiers
        .split(',')
        .map(|s| s.trim())
        .any(|admin| admin == phone)
}

pub async fn find_or_create_member(
    db: &PgPool,
    phone: &str,
    admin_identifiers: &str,
) -> Result<(Member, Identifier)> {
    let phone_hash = hash_phone_number(phone);
    let is_admin = is_admin_identifier(phone, admin_identifiers);

    // Try to find existing identifier
    let existing: Option<Identifier> = sqlx::query_as(
        "SELECT id, member_id, phone_hash, is_admin, created_at, updated_at FROM identifiers WHERE phone_hash = $1"
    )
    .bind(&phone_hash)
    .fetch_optional(db)
    .await?;

    if let Some(ident) = existing {
        let member: Member = sqlx::query_as(
            "SELECT id, created_at, last_active_at FROM members WHERE id = $1"
        )
        .bind(ident.member_id)
        .fetch_one(db)
        .await?;

        // Update last_active_at
        sqlx::query("UPDATE members SET last_active_at = now() WHERE id = $1")
            .bind(member.id)
            .execute(db)
            .await?;

        return Ok((member, ident));
    }

    // Create new member + identifier
    let member: Member = sqlx::query_as(
        "INSERT INTO members DEFAULT VALUES RETURNING id, created_at, last_active_at"
    )
    .fetch_one(db)
    .await?;

    let ident: Identifier = sqlx::query_as(
        "INSERT INTO identifiers (member_id, phone_hash, is_admin) VALUES ($1, $2, $3) RETURNING id, member_id, phone_hash, is_admin, created_at, updated_at"
    )
    .bind(member.id)
    .bind(&phone_hash)
    .bind(is_admin)
    .fetch_one(db)
    .await?;

    Ok((member, ident))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_phone_number() {
        let hash = hash_phone_number("+15551234567");
        assert_eq!(hash.len(), 64);
        assert_eq!(hash, hash_phone_number("+15551234567"));
        assert_ne!(hash, hash_phone_number("+15559999999"));
    }

    #[test]
    fn test_is_admin_identifier() {
        let admins = "+15551111111, +15552222222";
        assert!(is_admin_identifier("+15551111111", admins));
        assert!(is_admin_identifier("+15552222222", admins));
        assert!(!is_admin_identifier("+15553333333", admins));
        assert!(!is_admin_identifier("+15551111111", ""));
    }
}
