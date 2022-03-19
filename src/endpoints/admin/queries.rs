use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{db::tables, error::Result, Error};

use super::{AdminRow, Pagination};

#[instrument(skip(db))]
pub async fn get_address_admin_rows(pagination: &Pagination, db: &PgPool) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
            select
                address_id as pk,
                concat_ws(', ',
                    street_address1,
                    city,
                    state_province_county,
                    postal_code
                ) "name!",
                created_at,
                updated_at
            from address
            limit $1
            offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}

#[instrument(skip(db))]
pub async fn get_article_admin_rows(pagination: &Pagination, db: &PgPool) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
            select
                article_id as pk,
                title "name!",
                created_at,
                updated_at
            from article
            limit $1
            offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}

#[instrument(skip(db))]
pub async fn get_address_detail(pk: Uuid, db: &PgPool) -> Result<Option<tables::address::Address>> {
    sqlx::query_as!(
        tables::address::Address,
        r#"
            select
                address_id "address_id: tables::address::AddressId",
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
                etag "etag: tables::Etag"
            from address
            where address_id = $1
        "#,
        pk
    )
    .fetch_optional(db)
    .await
    .map_err(Error::Sqlx)
}


#[instrument(skip(db))]
pub async fn get_auction_admin_rows(pagination: &Pagination, db: &PgPool) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
            select
                auction_id as pk,
                title as name,
                created_at,
                updated_at
            from auction
            limit $1
            offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}
#[instrument(skip(db))]
pub async fn get_auction_item_admin_rows(
    pagination: &Pagination,
    db: &PgPool,
) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
            select
                auction_item_id as pk,
                title as name,
                created_at,
                updated_at
            from auction_item
            limit $1
            offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}
#[instrument(skip(db))]
pub async fn get_auction_item_bid_admin_rows(
    pagination: &Pagination,
    db: &PgPool,
) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
            select
                aib.auction_item_bid_id as "pk!",
                us.email as "name!",
                aib.created_at "created_at!",
                aib.updated_at "updated_at!"
            from auction_item_bid aib
            inner join "user" us
            on us.user_id = aib.user_id
            limit $1
            offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}
#[instrument(skip(db))]
pub async fn get_auction_item_delivery_admin_rows(
    pagination: &Pagination,
    db: &PgPool,
) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
            select
                aib.auction_item_bid_id as "pk!",
                us.email as "name!",
                aib.created_at "created_at!",
                aib.updated_at "updated_at!"
            from auction_item_bid aib
            inner join "user" us
            on us.user_id = aib.user_id
            limit $1
            offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}
#[instrument(skip(db))]
pub async fn get_organization_admin_rows(
    pagination: &Pagination,
    db: &PgPool,
) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
        select
            organization_id as pk,
            name,
            created_at,
            updated_at
        from organization
        limit $1
        offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}
#[instrument(skip(db))]
pub async fn get_user_admin_rows(pagination: &Pagination, db: &PgPool) -> Result<Vec<AdminRow>> {
    sqlx::query_as!(
        AdminRow,
        r#"
        select
            user_id as pk,
            email as name,
            created_at,
            updated_at
        from "user"
        limit $1
        offset $2
        "#,
        i64::try_from(pagination.per_page).unwrap_or(30),
        i64::try_from(pagination.page * pagination.per_page).unwrap_or(0)
    )
    .fetch_all(db)
    .await
    .map_err(Error::Sqlx)
}

#[instrument(skip(db))]
pub async fn insert_address_from_form(
    address: tables::address::AddressFromForm,
    db: &PgPool,
) -> Result<tables::address::Address> {
    sqlx::query_as!(
        tables::address::Address,
        r#"
        insert into address (
                street_address1, street_address2, street_address3,
                city, state_province_county, postal_code,
                country_code, latitude, longitude
            )
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            returning
                address_id as "address_id: tables::address::AddressId",
                street_address1, street_address2, street_address3,
                city, state_province_county, postal_code,
                country_code, latitude, longitude, created_at, updated_at,
                etag "etag: tables::Etag"
        "#,
        address.street_address1,
        address.street_address2,
        address.street_address3,
        address.city,
        address.state_province_county,
        address.postal_code,
        address.country_code,
        address.latitude.map(|n| n.parse::<f64>().ok()).flatten(),
        address.longitude.map(|n| n.parse::<f64>().ok()).flatten()
    )
    .fetch_one(db)
    .await
    .map_err(Error::Sqlx)
}
