use serde::de;
use sqlx::types::time::OffsetDateTime;
use std::fmt;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

pub mod address;
pub mod article;
pub mod auction;
pub mod organization;
pub mod user;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, sqlx::Type)]
pub struct Etag(pub Uuid);

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Table {
    Address,
    Article,
    Auction,
    AuctionItem,
    AuctionItemBid,
    AuctionItemDelivery,
    Organization,
    User,
}
impl Table {
    pub fn get_table_list() -> Vec<Table> {
        vec![
            Table::Address,
            Table::Article,
            Table::Auction,
            Table::AuctionItem,
            Table::AuctionItemBid,
            Table::AuctionItemDelivery,
            Table::Organization,
            Table::User,
        ]
    }
    pub fn to_url_name(&self) -> &str {
        match self {
            Table::Address => "address",
            Table::Article => "article",
            Table::Auction => "auction",
            Table::AuctionItem => "auction-item",
            Table::AuctionItemBid => "auction-item-bid",
            Table::AuctionItemDelivery => "auction-item-delivery",
            Table::Organization => "organization",
            Table::User => "user",
        }
    }

    pub fn to_postgres_name(&self) -> &str {
        match self {
            Table::Address => "address",
            Table::Article => "article",
            Table::Auction => "auction",
            Table::AuctionItem => "auction_item",
            Table::AuctionItemBid => "auction_item_bid",
            Table::AuctionItemDelivery => "auction_item_delivery",
            Table::Organization => "organization",
            Table::User => "user",
        }
    }
}
impl fmt::Display for Table {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Table::Address => write!(f, "Address"),
            Table::Article => write!(f, "Article"),
            Table::Auction => write!(f, "Auction"),
            Table::AuctionItem => write!(f, "Auction Item"),
            Table::AuctionItemBid => write!(f, "Auction Item Bid"),
            Table::AuctionItemDelivery => write!(f, "Auction Item Delivery"),
            Table::Organization => write!(f, "Organization"),
            Table::User => write!(f, "User"),
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Table::Organization
    }
}
