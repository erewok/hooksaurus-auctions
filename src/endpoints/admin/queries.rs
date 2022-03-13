use sqlx::PgPool;

use crate::db::tables;
use crate::{error::Result, Error};

use super::{AdminRow, AsAdminRow, Pagination};

impl AsAdminRow for tables::address::Address {
    fn to_admin_row(&self) -> AdminRow {
        AdminRow {
            pk: self.address_id.clone(),
            name: format!(
                "{}, {}, {}, {}",
                self.street_address1,
                self.city,
                self.state_province_county,
                self.postal_code.clone().unwrap_or_else(|| "".to_string())
            ),
            created_at: self.created_at.clone(),
            updated_at: self.updated_at.clone(),
        }
    }
}

pub async fn get_address_admin_rows(pagination: &Pagination, db: &PgPool) -> Result<Vec<AdminRow>> {
    let rows = sqlx::query_as!(
        tables::address::Address,
        r#"
            select 
                address_id, 
                street_address1, 
                street_address2,
                street_address3, 
                city, 
                state_province_county, 
                postal_code, 
                country_code, 
                latitude, 
                longitude, 
                created_at, 
                updated_at,
                etag
            from address
            limit $1
            offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)?
    .iter()
    .map(|r| r.to_admin_row())
    .collect();

    Ok(rows)
}
