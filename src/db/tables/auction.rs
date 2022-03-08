use sqlx::types::{Decimal, time::OffsetDateTime};
use uuid::Uuid;
use crate::db::tables;


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Auction {
    auction_id: Uuid,
    title: String,
    description: String,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    start_date: OffsetDateTime,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    end_date: OffsetDateTime,
    benefits_organization_id: Option<Uuid>,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    created_at: OffsetDateTime,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    updated_at: OffsetDateTime,
    etag: Uuid
}


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItem {
    auction_item_id: Uuid,
    // relates to this auction
    auction_id: Uuid,
    // This may be foreign-keyed to _another_ AuctionItem, which is called its "basket"
    basket_id: Uuid,

    // Monetary amounts relating to this item
    expected_retail_value: Decimal,
    minimum_bid_amount: Decimal,
    buy_it_now_amount: Option<Decimal>,

    // Metadata
    title: String,
    description: String,
    featured_image_filepath: String,
    image_dir: String,
    tag_list: Vec<String>,
    donated_by_organization_id: Uuid,
    benefits_organization_id: Uuid,

    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    active_start_date: OffsetDateTime,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    active_end_date: OffsetDateTime,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    created_at: OffsetDateTime,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    updated_at: OffsetDateTime,
    etag: Uuid
}


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemBid {
    auction_item_bid_id: Uuid,
    // relates to this auction_item
    auction_item_id: Uuid,
    // User who made this bid
    user_id: Uuid,

    // Monetary amounts relating to this bid
    amount: Decimal,
    max_bid_amount: Option<Decimal>,

    // set after auction ends
    is_winning_bid: bool,

    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    created_at: OffsetDateTime,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    updated_at: OffsetDateTime,
    etag: Uuid
}

/// Represents a delivery request for this auction item
/// It is expected that shipping will be calculated for the buyer's address
/// This table foreign-keys to address as a result
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemDelivery {
    // Bid this delivery relates to
    auction_item_bid_id: Uuid,
    // User who made this bid
    user_id: Uuid,
    // Shipping address for delivery
    shipping_address: Uuid,
    shipping_fee: Option<Decimal>,
    #[serde(deserialize_with = "tables::deserialize_optional_datetime", serialize_with = "tables::serialize_option_dt")]
    shipped_datetime: Option<OffsetDateTime>,
    #[serde(deserialize_with = "tables::deserialize_optional_datetime", serialize_with = "tables::serialize_option_dt")]
    delivered: Option<OffsetDateTime>,
    // columns below are relating to storing delivery info and exceptions
    shipping_exception: Option<String>,
    sms_updates_number: Option<String>,
    email_contact: Option<String>,
    signature_name: Option<String>,
    signed_for_by: Option<String>,
    carrier: Option<String>,
    tracking_number: Option<String>,

    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    created_at: OffsetDateTime,
    #[serde(deserialize_with = "tables::deserialize_dt", serialize_with = "tables::serialize_dt")]
    updated_at: OffsetDateTime,
    etag: Uuid
}