use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

mod handlers;
mod queries;

use crate::db::tables::{self, serialize_dt};
pub use handlers::router;

#[derive(Clone, Debug, serde::Deserialize)]
pub struct Pagination {
    pub page: usize,
    pub per_page: usize,
}
impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: 30,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct AdminRow {
    pub pk: Uuid,
    pub name: String,
    #[serde(serialize_with = "serialize_dt")]
    pub created_at: OffsetDateTime,
    #[serde(serialize_with = "serialize_dt")]
    pub updated_at: OffsetDateTime,
}
