use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use crate::db::tables::{deserialize_dt, serialize_dt};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Address {
    address_id: Uuid,
    street_address1: String,
    street_address2: Option<String>,
    street_address3: Option<String>,
    city: String,
    state_province_county: String,
    postal_code: Option<String>,
    country_code: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    created_at: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    updated_at: OffsetDateTime,
    etag: Uuid
}
