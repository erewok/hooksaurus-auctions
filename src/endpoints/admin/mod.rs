use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

mod handlers;

use crate::db::tables::{deserialize_dt, serialize_dt};
pub use handlers::router;

#[derive(serde::Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}
impl Default for Pagination {
    fn default() -> Self {
        Self { page: 0, per_page: 30 }
    }
}

#[derive(Debug, serde::Serialize)]
struct Row {
    pub pk: Uuid,
    pub name: String,
    #[serde(serialize_with = "serialize_dt")]
    pub created: OffsetDateTime,
    #[serde(serialize_with = "serialize_dt")]
    pub modified: OffsetDateTime,
}