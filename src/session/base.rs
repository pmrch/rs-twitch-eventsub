use super::{DateTime, Deserialize, Utc, Uuid};

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum MessageId {
    StringId(String),
    UuidId(Uuid),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct BaseMetadata {
    #[serde(deserialize_with = "super::deserialize_message_id")]
    pub message_id: MessageId,
    pub message_type: String,
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub message_timestamp: DateTime<Utc>,

    // Optional fields for notification events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_version: Option<String>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct BaseEventMessage<T> {
    pub metadata: BaseMetadata,
    pub payload: T,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Transport {
    pub method: String,
    pub session_id: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Subscription {
    pub id: Uuid,
    pub status: String,
    #[serde(rename = "type")]
    pub sub_type: String,
    pub version: String,
    pub cost: u32,
    pub condition: serde_json::Value,
    pub transport: Transport,
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub created_at: DateTime<Utc>,
}
