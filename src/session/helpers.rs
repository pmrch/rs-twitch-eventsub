use serde::Deserializer;

use super::{DateTime, Deserialize, MessageId, Utc, Uuid};

pub fn from_rfc3339<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>, {
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<DateTime<Utc>>().map_err(serde::de::Error::custom)
}

pub fn deserialize_message_id<'de, D>(deserializer: D) -> Result<MessageId, D::Error>
where
    D: Deserializer<'de>, {
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(s.parse::<Uuid>().map_or_else(|_| MessageId::StringId(s), MessageId::UuidId))
}
