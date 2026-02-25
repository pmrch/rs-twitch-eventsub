use super::{DateTime, Deserialize, MessageId, Utc};

/// An object that identifies the message.
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
#[allow(clippy::struct_field_names)]
pub struct ReconnectMetadata {
    #[serde(deserialize_with = "super::deserialize_message_id")]
    /// An ID that uniquely identifies the message. Twitch sends messages at
    /// least once, but if Twitch is unsure of whether you received a
    /// notification, it’ll resend the message. This means you may receive a
    /// notification twice. If Twitch resends the message, the message ID will
    /// be the same.
    pub message_id: MessageId,
    /// The type of message, which is set to `session_reconnect`.
    pub message_type: String,
    /// The UTC date and time that the message was sent.
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub message_timestamp: DateTime<Utc>,
}

/// An object that contains information about the connection.
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct ReconnectSession {
    /// An ID that uniquely identifies this WebSocket connection.
    pub id: String,
    /// The connection’s status, which is set to reconnecting.
    pub status: String,
    /// Is set to null.
    pub keepalive_timeout_seconds: Option<u32>,
    /// The URL to reconnect to. Use this URL as is; do not modify it. The
    /// connection automatically includes the subscriptions from the old
    /// connection.
    pub reconnect_url: String,
    /// The UTC date and time when the connection was created.
    #[serde(deserialize_with = "super::from_rfc3339")]
    pub connected_at: DateTime<Utc>,
}

/// An object that contains the message.
#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct ReconnectPayload {
    pub session: ReconnectSession,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct ReconnectMessage {
    pub metadata: ReconnectMetadata,
    pub payload: ReconnectPayload,
}
