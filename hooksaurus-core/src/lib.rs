use time::OffsetDateTime;
use uuid::Uuid;

pub mod datetimes;

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
    #[serde(serialize_with = "crate::datetimes::serialize_dt")]
    pub created_at: OffsetDateTime,
    #[serde(serialize_with = "crate::datetimes::serialize_dt")]
    pub updated_at: OffsetDateTime,
}

pub trait ToForm {
    fn to_form(&self) -> String;
    fn to_empty_form() -> String;
}

pub trait FromForm {
    fn to_form(&self) -> String;
    fn to_empty_form() -> String;
}
