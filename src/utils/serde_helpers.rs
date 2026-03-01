use serde::Deserializer;

use super::{DateTime, Deserialize, MessageId, Utc, Uuid};

/// Deserializes an RFC 3339 timestamp string into a [`DateTime<Utc>`].
///
/// # Errors
///
/// - Returns a deserialization error if the input is not a valid UTF-8 string
/// - Returns a deserialization error if the string cannot be parsed as an RFC
///   3339 timestamp
pub fn from_rfc3339<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>, {
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<DateTime<Utc>>().map_err(serde::de::Error::custom)
}

/// Deserializes a message ID string into a [`MessageId`].
///
/// Attempts to parse the string as a [`Uuid`]. If parsing succeeds, returns
/// [`MessageId::UuidId`], otherwise falls back to [`MessageId::StringId`].
///
/// # Errors
///
/// - Returns a deserialization error if the input cannot be deserialized as a
///   string
pub fn deserialize_message_id<'de, D>(deserializer: D) -> Result<MessageId, D::Error>
where
    D: Deserializer<'de>, {
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(s.parse::<Uuid>().map_or_else(|_| MessageId::StringId(s), MessageId::UuidId))
}
