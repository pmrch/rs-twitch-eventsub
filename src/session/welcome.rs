use super::{BaseMetadata, DateTime, Deserialize, MessageId, Utc};

#[derive(Deserialize, Debug)]
pub struct WelcomeMetadata {
    #[serde(deserialize_with = "super::deserialize_message_id")]
    pub message_id: MessageId,
    pub message_type: String,
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub message_timestamp: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct WelcomeSession {
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub connected_at: DateTime<Utc>,
    pub id: String,
    pub keepalive_timeout_seconds: u32,
    pub reconnect_url: Option<String>,
    pub recovery_url: Option<String>,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct WelcomePayload {
    pub session: Option<WelcomeSession>,
}

#[derive(Deserialize, Debug)]
pub struct WelcomeMessage {
    pub metadata: WelcomeMetadata,
    pub payload: WelcomePayload,
}

impl From<BaseMetadata> for WelcomeMetadata {
    fn from(metadata: BaseMetadata) -> Self {
        Self {
            message_id: metadata.message_id,
            message_type: metadata.message_type,
            message_timestamp: metadata.message_timestamp,
        }
    }
}

impl WelcomeMessage {
    #[must_use]
    pub fn from_base(metadata: BaseMetadata, payload: WelcomePayload) -> Self {
        Self { metadata: metadata.into(), payload }
    }
}
