use super::{BaseMetadata, DateTime, Deserialize, MessageId, Subscription, Utc};

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct RevocationMetadata {
    /// An ID that uniquely identifies the message. Twitch sends messages at
    /// least once, but if Twitch is unsure of whether you received a
    /// notification, it’ll resend the message. This means you may receive a
    /// notification twice. If Twitch resends the message, the message ID will
    /// be the same.
    #[serde(deserialize_with = "super::deserialize_message_id")]
    pub message_id: MessageId,

    /// The type of message, which is set to revocation.
    pub message_type: String,

    /// The UTC date and time that the message was sent.
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub message_timestamp: DateTime<Utc>,

    /// The type of event sent in the message.
    pub subscription_type: String,

    /// The version number of the subscription type's definition. This is the
    /// same value specified in the subscription request.
    pub subscription_version: String,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct RevocationPayload {
    /// An object that contains information about your subscription.
    pub subscription: Subscription,
}

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct RevocationMessage {
    /// An object that identifies the message.
    pub metadata: RevocationMetadata,

    /// An object that contains the message.
    pub payload: RevocationPayload,
}

impl TryFrom<BaseMetadata> for RevocationMetadata {
    type Error = anyhow::Error;

    fn try_from(metadata: BaseMetadata) -> Result<Self, Self::Error> {
        let sub_type: String = metadata.subscription_type.ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to parse BaseMetadata for RevocationMetadata, subscription_type was None"
            )
        })?;

        let sub_ver: String = metadata.subscription_version.ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to parse BaseMetadata for RevocationMetadata, subscription_type was None"
            )
        })?;

        Ok(Self {
            message_id: metadata.message_id,
            message_type: metadata.message_type,
            message_timestamp: metadata.message_timestamp,
            subscription_type: sub_type,
            subscription_version: sub_ver,
        })
    }
}

impl RevocationMessage {
    /// Converts `BaseMetadata` and payload to `RevocationMessage`
    ///
    /// # Errors
    ///
    /// - Returns `anyhow::Error` if invalid metadata was passed to this
    ///   function
    pub fn from_base(
        metadata: BaseMetadata,
        payload: RevocationPayload,
    ) -> crate::prelude::Result<Self> {
        Ok(Self { metadata: metadata.try_into()?, payload })
    }
}
