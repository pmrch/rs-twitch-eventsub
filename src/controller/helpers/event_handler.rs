use super::super::{
    Arc, BaseEventMessage, Error, KeepaliveMessage, KeepalivePayload, NotificationMessage,
    NotificationPayload, ReconnectMessage, ReconnectPayload, Result, RevocationMessage,
    RevocationPayload, WelcomeMessage, WelcomePayload
};

use super::Subscriber;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum EventType {
    ChatMessage,
    Subscription,
    Bits,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bits => write!(f, "bits"),
            Self::ChatMessage => write!(f, "chat_message"),
            Self::Subscription => write!(f, "subscription"),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum EventMessage {
    Welcome(WelcomeMessage),
    Keepalive(KeepaliveMessage),
    Notification(NotificationMessage),
    Reconnect(ReconnectMessage),
    Revocation(RevocationMessage),
    None,
}

/// This function handles each event by parsing type and extracting
/// appropriately.
///
/// # Errors
///
/// - Returns `serde_json::Error` if parsing a `serde_json::Value` into a chosen
///   type fails, or if deserializing the event JSON fails for some reason
/// - Returns `anyhow::Error` if session ID was not set during the welcome event
/// - Returns `reqwest::Error` if subscribing to the event was unsuccessful
pub async fn handle_event(
    raw: &str,
    is_reconnect: bool,
    subscriber: Arc<Subscriber>
) -> Result<EventMessage> {
    tracing::trace!("Handling event: {raw}");
    let peek: BaseEventMessage<serde_json::Value> = serde_json::from_str(raw)?;
    match peek.metadata.message_type.as_str() {
        "session_welcome" => {
            let payload: WelcomePayload = serde_json::from_value(peek.payload)?;
            let msg: WelcomeMessage = WelcomeMessage::from_base(peek.metadata, payload);

            if let Some(session) = &msg.payload.session {
                subscriber.set_session_id(&session.id).await;
                tracing::debug!("Got session_welcome.");
            }

            if is_reconnect {
                tracing::info!("Reconnect welcome received, skipping subscription");
            } else {
                
            }

            Ok(EventMessage::Welcome(msg))
        }
        "session_keepalive" => match serde_json::from_value::<KeepalivePayload>(peek.payload) {
            Ok(payload) => {
                let msg: KeepaliveMessage = KeepaliveMessage::from_base(peek.metadata, payload);
                tracing::trace!("Got session_keepalive message");
                Ok(EventMessage::Keepalive(msg))
            }
            Err(e) => {
                tracing::error!("Failed to parse keepalive: {e}");
                Err(Error::SerdeError(e))
            }
        },
        "notification" => match serde_json::from_value::<NotificationPayload>(peek.payload) {
            Ok(payload) => {
                let msg: NotificationMessage =
                    NotificationMessage::from_base(peek.metadata, payload);

                tracing::trace!("Got notification message");
                Ok(EventMessage::Notification(msg))
            }
            Err(e) => {
                tracing::error!("Failed to parse notification: {e}");
                Err(Error::SerdeError(e))
            }
        },
        "session_reconnect" => match serde_json::from_value::<ReconnectPayload>(peek.payload) {
            Ok(payload) => {
                let msg: ReconnectMessage = ReconnectMessage::from_base(peek.metadata, payload);
                tracing::info!("Got session_reconnect message");
                Ok(EventMessage::Reconnect(msg))
            }
            Err(e) => {
                tracing::error!("Failed to parse reconnect: {e}");
                Err(Error::SerdeError(e))
            }
        },
        "revocation" => match serde_json::from_value::<RevocationPayload>(peek.payload) {
            Ok(payload) => {
                let msg: RevocationMessage = RevocationMessage::from_base(peek.metadata, payload)?;
                tracing::info!("Got revocation message");
                Ok(EventMessage::Revocation(msg))
            }
            Err(e) => {
                tracing::error!("Failed to parse revocation: {e}");
                Err(Error::SerdeError(e))
            }
        },
        _ => {
            tracing::warn!("Non-registered event was received");
            Ok(EventMessage::None)
        }
    }
}
