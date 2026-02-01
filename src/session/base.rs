use super::{DateTime, Deserialize, Utc, Uuid};

#[derive(Debug)]
pub enum MessageId {
    StringId(String),
    UuidId(Uuid),
}

#[derive(Deserialize, Debug)]
pub struct BaseMetadata {
    #[serde(deserialize_with = "super::deserialize_message_id")]
    pub message_id: MessageId,
    pub message_type: String,
    pub message_timestamp: DateTime<Utc>,

    // Optional fields for notification events
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_version: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct BaseEventMessage<T> {
    pub metadata: BaseMetadata,
    pub payload: T,
}
