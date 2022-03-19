use crate::db::tables::{deserialize_dt, serialize_dt};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum OrgType {
    Business,
    FarmAnimalSanctuary,
    NonProfit,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]

pub struct OrganizationId(pub Uuid);
impl std::fmt::Display for OrganizationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Organization {
    pub organization_id: OrganizationId,
    pub org_type: OrgType,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub email: String,
    pub website: String,
    pub contact_name: Option<String>,
    pub phone_number: Option<String>,
    pub alt_phone_number: Option<String>,
    pub primary_address_id: super::address::AddressId,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub created_at: OffsetDateTime,
    #[serde(deserialize_with = "deserialize_dt", serialize_with = "serialize_dt")]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}
