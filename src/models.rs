use serde::{Deserialize, Serialize};
use rusqlite::{types::{FromSql, FromSqlResult, ValueRef, ToSql, ToSqlOutput}, Result as SqliteResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub api_key: Option<String>,
    pub created_at: i64,
    pub last_login: Option<i64>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceAlert {
    pub id: Option<i64>,
    pub user_id: i64,
    pub symbol: String,
    pub target_price: f64,
    pub condition: AlertCondition,
    pub created_at: i64,
    pub triggered_at: Option<i64>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i64,
    pub user_id: i64,
    pub key: String,
    pub created_at: i64,
    pub last_used: Option<i64>,
    pub expires_at: Option<i64>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertCondition {
    Above,
    Below,
}

impl FromSql for AlertCondition {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let text = value.as_str()?;
        match text {
            "Above" => Ok(AlertCondition::Above),
            "Below" => Ok(AlertCondition::Below),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for AlertCondition {
    fn to_sql(&self) -> SqliteResult<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(match self {
            AlertCondition::Above => "Above",
            AlertCondition::Below => "Below",
        }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoPrice {
    pub symbol: String,
    pub price: f64,
    pub exchange: String,
    pub timestamp: i64,
} 