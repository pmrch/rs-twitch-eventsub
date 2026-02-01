use super::{BaseMetadata, DateTime, Deserialize, MessageId, Utc};

#[derive(Deserialize, Debug)]
pub struct KeepaliveSession {}

#[derive(Deserialize, Debug)]
pub struct KeepaliveMetadata {
    #[serde(deserialize_with = "super::deserialize_message_id")]
    pub message_id: MessageId,
    pub message_type: String,
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub message_timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct KeepalivePayload {
    pub session: Option<KeepaliveSession>,
}

#[derive(Deserialize, Debug)]
pub struct KeepaliveMessage {
    pub metadata: KeepaliveMetadata,
    pub payload: KeepalivePayload,
}

impl From<BaseMetadata> for KeepaliveMetadata {
    fn from(metadata: BaseMetadata) -> Self {
        Self {
            message_id: metadata.message_id,
            message_type: metadata.message_type,
            message_timestamp: metadata.message_timestamp,
        }
    }
}

impl KeepaliveMessage {
    #[must_use]
    pub fn from_base(metadata: BaseMetadata, payload: KeepalivePayload) -> Self {
        Self { metadata: metadata.into(), payload }
    }
}
