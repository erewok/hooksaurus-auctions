use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use crate::db::tables::{deserialize_dt, serialize_dt};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum OrgType {
    Business,
    FarmAnimalSanctuary,
    NonProfit
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Organization {
    organization_id: Uuid,
    org_type: OrgType,
    name: String,
    description: Option<String>,
    image: Option<String>,
    email: String,
    website: String,
    contact_name: Option<String>,
    phone_number: Option<String>,
    alt_phone_number: Option<String>,
    primary_address_id: Uuid,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    created_at: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    updated_at: OffsetDateTime,
    etag: Uuid
}
