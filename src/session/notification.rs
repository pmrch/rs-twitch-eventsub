use super::{
    BaseMetadata, ChannelChatMessage, DateTime, Deserialize, MessageId, NotificationEvent, Utc,
    Uuid,
};

#[derive(Deserialize, Debug)]
pub struct NotificationMetadata {
    #[serde(deserialize_with = "super::deserialize_message_id")]
    pub message_id: MessageId,
    pub message_type: String,
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub message_timestamp: DateTime<Utc>,
    pub subscription_type: String,
    pub subscription_version: String,
}

#[derive(Deserialize, Debug)]
pub struct Transport {
    pub method: String,
    pub session_id: String,
}

#[derive(Deserialize, Debug)]
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
#[derive(Debug)]
pub struct NotificationPayload {
    pub subscription: Subscription,
    pub event: NotificationEvent,
}

#[derive(Deserialize, Debug)]
pub struct NotificationMessage {
    pub metadata: NotificationMetadata,
    pub payload: NotificationPayload,
}

impl From<BaseMetadata> for NotificationMetadata {
    fn from(metadata: BaseMetadata) -> Self {
        let err: &str = r"
            You STUPID piece of shit, non-notification metadata was passed 
            to a notification constructor, this panic!() was probably deserved
        ";

        let subscription_type: String = metadata.subscription_type.expect(err);
        let subscription_version: String = metadata.subscription_version.expect(err);

        Self {
            message_id: metadata.message_id,
            message_type: metadata.message_type,
            message_timestamp: metadata.message_timestamp,
            subscription_type,
            subscription_version,
        }
    }
}

impl<'de> Deserialize<'de> for NotificationPayload {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        #[derive(Deserialize)]
        struct RawPayload {
            subscription: Subscription,
            event: serde_json::Value,
        }

        let raw: RawPayload = RawPayload::deserialize(deserializer)?;
        let event: NotificationEvent = match raw.subscription.sub_type.as_str() {
            "channel.chat.message" => {
                let ccm: ChannelChatMessage =
                    serde_json::from_value(raw.event).map_err(serde::de::Error::custom)?;
                NotificationEvent::ChannelChatMessage(Box::new(ccm))
            }
            other => NotificationEvent::Other(serde_json::Value::String(other.to_string())),
        };

        Ok(Self { subscription: raw.subscription, event })
    }
}

impl NotificationMessage {
    #[must_use]
    pub fn from_base(metadata: BaseMetadata, payload: NotificationPayload) -> Self {
        Self { metadata: metadata.into(), payload }
    }
}
