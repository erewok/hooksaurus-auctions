use crate::db::tables::{deserialize_dt, serialize_dt};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct AddressId(pub Uuid);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Address {
    pub address_id: AddressId,
    pub street_address1: String,
    pub street_address2: Option<String>,
    pub street_address3: Option<String>,
    pub city: String,
    pub state_province_county: String,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub created_at: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AddressFromForm {
    pub street_address1: String,
    pub street_address2: Option<String>,
    pub street_address3: Option<String>,
    pub city: String,
    pub state_province_county: String,
    pub postal_code: Option<String>,
    pub country_code: Option<String>,
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}
