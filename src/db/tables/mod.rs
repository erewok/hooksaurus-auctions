use serde::de;
use sqlx::types::time::OffsetDateTime;
use std::fmt;

pub mod address;
pub mod article;
pub mod auction;
pub mod organization;
pub mod user;

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
        match OffsetDateTime::parse(value, "%d-%b-%Y %H:%M:%SZ") {
            Ok(odt) => Ok(odt),
            Err(e) => Err(E::custom(format!("Parse error {} for {}", e, value))),
        }
    }
}

// Custom datetime serializer
pub fn serialize_dt<S: serde::Serializer>(
    dt: &OffsetDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(dt.format("%Y-%m-%d %H:%M:%S.0000z").as_str())
}
