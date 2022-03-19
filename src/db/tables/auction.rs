use crate::db::tables;
use sqlx::types::{time::OffsetDateTime, Decimal};
use uuid::Uuid;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct AuctionId(pub Uuid);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Auction {
    pub auction_id: AuctionId,
    pub title: String,
    pub description: String,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub start_date: OffsetDateTime,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub end_date: OffsetDateTime,
    pub benefits_organization_id: Option<super::organization::OrganizationId>,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct AuctionItemId(pub Uuid);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItem {
    pub auction_item_id: AuctionItemId,
    // relates to this auction
    pub auction_id: AuctionId,
    // This may be foreign-keyed to _another_ AuctionItem, which is called its "basket"
    pub basket_id: Option<AuctionItemId>,

    // Monetary amounts relating to this item
    pub expected_retail_value: Decimal,
    pub minimum_bid_amount: Decimal,
    pub buy_it_now_amount: Option<Decimal>,

    // Metadata
    pub title: String,
    pub description: String,
    pub featured_image_filepath: String,
    pub image_dir: String,
    pub tag_list: Vec<String>,
    pub donated_by_organization_id: Option<super::organization::OrganizationId>,
    pub benefits_organization_id: Option<super::organization::OrganizationId>,

    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub active_start_date: OffsetDateTime,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub active_end_date: OffsetDateTime,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct AuctionItemBidId(pub Uuid);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemBid {
    auction_item_bid_id: AuctionItemBidId,
    // relates to this auction_item
    auction_item_id: AuctionItemId,
    // User who made this bid
    user_id: Uuid,

    // Monetary amounts relating to this bid
    amount: Decimal,
    max_bid_amount: Option<Decimal>,

    // set after auction ends
    is_winning_bid: bool,

    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

/// Represents a delivery request for this auction item
/// It is expected that shipping will be calculated for the buyer's address
/// This table foreign-keys to address as a result
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemDelivery {
    // Bid this delivery relates to
    auction_item_bid_id: AuctionItemBidId,
    // User who made this bid
    user_id: Uuid,
    // Shipping address for delivery
    shipping_address: super::address::AddressId,
    shipping_fee: Option<Decimal>,
    #[serde(
        deserialize_with = "tables::deserialize_optional_datetime",
        serialize_with = "tables::serialize_option_dt"
    )]
    shipped_datetime: Option<OffsetDateTime>,
    #[serde(
        deserialize_with = "tables::deserialize_optional_datetime",
        serialize_with = "tables::serialize_option_dt"
    )]
    delivered: Option<OffsetDateTime>,
    // columns below are relating to storing delivery info and exceptions
    shipping_exception: Option<String>,
    sms_updates_number: Option<String>,
    email_contact: Option<String>,
    signature_name: Option<String>,
    signed_for_by: Option<String>,
    carrier: Option<String>,
    tracking_number: Option<String>,

    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "tables::deserialize_dt",
        serialize_with = "tables::serialize_dt"
    )]
    updated_at: OffsetDateTime,
    pub etag: super::Etag,
}
