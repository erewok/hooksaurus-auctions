use serde::de;
use sqlx::types::time::OffsetDateTime;
use std::fmt;
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

// Custom datetime deserializer
struct DateTimeFromCustomFormatVisitor;

pub fn deserialize_dt<'de, D>(d: D) -> Result<OffsetDateTime, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_str(DateTimeFromCustomFormatVisitor)
}

impl<'de> de::Visitor<'de> for DateTimeFromCustomFormatVisitor {
    type Value = OffsetDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a datetime string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match OffsetDateTime::parse(value, "%Y-%m-%d %H:%M:%SZ") {
            Ok(odt) => Ok(odt),
            Err(e) => Err(E::custom(format!("Parse error {} for {}", e, value))),
        }
    }
}
// Custom datetime deserializer for Optional value
pub fn deserialize_optional_datetime<'de, D>(d: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: de::Deserializer<'de>,
{
    d.deserialize_option(OptionalDateTimeFromCustomFormatVisitor)
}
struct OptionalDateTimeFromCustomFormatVisitor;

impl<'de> de::Visitor<'de> for OptionalDateTimeFromCustomFormatVisitor {
    type Value = Option<OffsetDateTime>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null or a datetime string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        Ok(Some(d.deserialize_str(DateTimeFromCustomFormatVisitor)?))
    }
}

// Custom datetime serializer
pub fn serialize_option_dt<S: serde::Serializer>(
    odt: &Option<OffsetDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match odt {
        Some(dt) => serialize_dt(dt, serializer),
        None => serializer.serialize_none(),
    }
}

// Custom datetime serializer
pub fn serialize_dt<S: serde::Serializer>(
    dt: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(dt.format("%Y-%m-%d %H:%M:%SZ").as_str())
}
