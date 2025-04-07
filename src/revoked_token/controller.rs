use chrono::Utc;

use crate::core::DbPool;

use super::model::RevokedToken;

pub async fn is_token_revoked(token: &str, pool: &DbPool) -> Result<bool, sqlx::Error> {
    let token = sqlx::query_as::<_, RevokedToken>("SELECT * FROM revoked_tokens WHERE token = ?")
        .bind(token)
        .fetch_optional(pool)
        .await?;

    Ok(token.is_some())
}

pub async fn revoke_token(revoke: RevokedToken, pool: &DbPool) -> Result<(), sqlx::Error> {
    let _ = sqlx::query("INSERT INTO revoked_tokens (token, expiration_date) VALUES (?, ?)")
        .bind(revoke.token)
        .bind(revoke.expiration_date)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn maintainance(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Get now date, all tokens that are expired will be purged from the database
    let now = Utc::now().naive_utc();

    let _ = sqlx::query("DELETE FROM revoked_tokens WHERE expiration_date < ?")
        .bind(now)
        .execute(pool)
        .await?;
    Ok(())
}
