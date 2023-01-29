use serde::de;
use std::fmt;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

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
        match OffsetDateTime::parse(value, &Rfc3339) {
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

// Custom datetime serializer for Option
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
    serializer.serialize_str(dt.format(&Rfc3339).unwrap_or_default().as_str())
}
