use crate::application::models::transaction::Transaction;
use crate::error::AppError;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use sqlx::Executor;

/// Stores a list of transactions in the database
///
/// # Arguments
/// * `pool` - PostgreSQL connection pool
/// * `txs` - List of transactions to store
///
/// # Returns
/// * `Result<usize, AppError>` - Number of transactions inserted or an error
pub async fn store_transactions(
    pool: &sqlx::PgPool,
    txs: &[Transaction],
) -> Result<usize, AppError> {
    let mut tx = pool.begin().await?;
    let mut inserted = 0;

    for t in txs {
        let result = tx
            .execute(
                sqlx::query(
                    r#"
                    INSERT INTO ig_options (
                        reference, deal_date, underlying, strike,
                        option_type, expiry, transaction_type, pnl_eur, is_fee, raw
                    )
                    VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
                    ON CONFLICT (raw_hash) DO NOTHING
                    "#,
                )
                .bind(&t.reference)
                .bind(t.deal_date)
                .bind(&t.underlying)
                .bind(t.strike)
                .bind(&t.option_type)
                .bind(t.expiry)
                .bind(&t.transaction_type)
                .bind(t.pnl_eur)
                .bind(t.is_fee)
                .bind(&t.raw_json),
            )
            .await?;

        inserted += result.rows_affected() as usize;
    }

    tx.commit().await?;
    Ok(inserted)
}

/// Serializes a value to a JSON string
pub fn serialize_to_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

/// Deserializes a JSON string into a value
pub fn deserialize_from_json<T: DeserializeOwned>(s: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(s)
}
