use crate::db::tables;
use sqlx::types::{time::OffsetDateTime, Decimal};
use uuid::Uuid;

/// Auction is the umbrella for these other tables
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct AuctionId(pub Uuid);
impl std::fmt::Display for AuctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Auction {
    pub auction_id: AuctionId,
    pub title: String,
    pub description: String,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub start_date: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub end_date: OffsetDateTime,
    pub benefits_organization_id: Option<super::organization::OrganizationId>,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionFromForm {
    pub title: String,
    pub description: String,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub start_date: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub end_date: OffsetDateTime,
    pub benefits_organization_id: Option<super::organization::OrganizationId>,
}

/// An Auction is composed of one or more AuctionItems
/// In addition, an AuctionItem can be part of a "basket",
/// which means a group of AuctionItems that all foreign-key to another

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct AuctionItemId(pub Uuid);

impl std::fmt::Display for AuctionItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

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
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub active_start_date: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub active_end_date: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemFromForm {
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
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub active_start_date: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub active_end_date: OffsetDateTime,
}

/// An AuctionItemBid represents a bid by a single person for a particular
/// auction AuctionItem
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct AuctionItemBidId(pub Uuid);

impl std::fmt::Display for AuctionItemBidId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemBid {
    pub auction_item_bid_id: AuctionItemBidId,
    // relates to this auction_item
    pub auction_item_id: AuctionItemId,
    // User who made this bid
    pub user_id: Uuid,

    // Monetary amounts relating to this bid
    pub amount: Decimal,
    pub max_bid_amount: Option<Decimal>,

    // set after auction ends
    pub is_winning_bid: bool,

    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemBidFromForm {
    // relates to this auction_item
    pub auction_item_id: AuctionItemId,
    // User who made this bid
    pub user_id: Uuid,

    // Monetary amounts relating to this bid
    pub amount: Decimal,
    pub max_bid_amount: Option<Decimal>,

    // set after auction ends
    pub is_winning_bid: bool,
}

/// AuctionItemDelivery Represents a delivery request for this auction item
/// It is expected that shipping will be calculated for the buyer's address
/// This table foreign-keys to address as a result
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemDelivery {
    // Bid this delivery relates to
    pub auction_item_bid_id: AuctionItemBidId,
    // User who made this bid
    pub user_id: Uuid,
    // Shipping address for delivery
    pub shipping_address: super::address::AddressId,
    pub shipping_fee: Option<Decimal>,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_optional_datetime",
        serialize_with = "hooksaurus_core::datetimes::serialize_option_dt"
    )]
    pub shipped_datetime: Option<OffsetDateTime>,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_optional_datetime",
        serialize_with = "hooksaurus_core::datetimes::serialize_option_dt"
    )]
    pub delivered: Option<OffsetDateTime>,
    // columns below are relating to storing delivery info and exceptions
    pub shipping_exception: Option<String>,
    pub sms_updates_number: Option<String>,
    pub email_contact: Option<String>,
    pub signature_name: Option<String>,
    pub signed_for_by: Option<String>,
    pub carrier: Option<String>,
    pub tracking_number: Option<String>,

    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub created_at: OffsetDateTime,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_dt",
        serialize_with = "hooksaurus_core::datetimes::serialize_dt"
    )]
    pub updated_at: OffsetDateTime,
    pub etag: super::Etag,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AuctionItemDeliveryFromForm {
    // Bid this delivery relates to
    pub auction_item_bid_id: AuctionItemBidId,
    // User who made this bid
    pub user_id: Uuid,
    // Shipping address for delivery
    pub shipping_address: super::address::AddressId,
    pub shipping_fee: Option<Decimal>,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_optional_datetime",
        serialize_with = "hooksaurus_core::datetimes::serialize_option_dt"
    )]
    pub shipped_datetime: Option<OffsetDateTime>,
    #[serde(
        deserialize_with = "hooksaurus_core::datetimes::deserialize_optional_datetime",
        serialize_with = "hooksaurus_core::datetimes::serialize_option_dt"
    )]
    pub delivered: Option<OffsetDateTime>,
    // columns below are relating to storing delivery info and exceptions
    pub shipping_exception: Option<String>,
    pub sms_updates_number: Option<String>,
    pub email_contact: Option<String>,
    pub signature_name: Option<String>,
    pub signed_for_by: Option<String>,
    pub carrier: Option<String>,
    pub tracking_number: Option<String>,
}
