use super::super::{
    Arc, BaseEventMessage, Client, Error, KeepaliveMessage, KeepalivePayload, NotificationMessage,
    NotificationPayload, Result, RwLock, UserConfig, WelcomeMessage, WelcomePayload,
};
use super::subscribe_to_chat;

#[allow(unused)]
#[derive(Debug)]
pub enum EventMessage {
    Welcome(WelcomeMessage),
    Keepalive(KeepaliveMessage),
    Notification(NotificationMessage),
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
    session_id: Arc<RwLock<Option<String>>>,
    http_client: Arc<Client>,
    user_config: &UserConfig,
) -> Result<EventMessage> {
    tracing::debug!("Handling event: {raw}");
    let peek: BaseEventMessage<serde_json::Value> = serde_json::from_str(raw)?;

    match peek.metadata.message_type.as_str() {
        "session_welcome" => {
            let payload: WelcomePayload = serde_json::from_value(peek.payload)?;
            let msg: WelcomeMessage = WelcomeMessage::from_base(peek.metadata, payload);

            if let Some(session) = &msg.payload.session {
                *session_id.write().await = Some(session.id.clone());
                tracing::debug!("Got session_welcome.");
            }

            let session_id: String = session_id
                .read()
                .await
                .clone()
                .ok_or_else(|| Error::NoneError("Tried to read None session ID".into()))?;

            subscribe_to_chat(http_client, &session_id, user_config).await?;
            Ok(EventMessage::Welcome(msg))
        }
        "session_keepalive" => match serde_json::from_value::<KeepalivePayload>(peek.payload) {
            Ok(payload) => {
                let msg: KeepaliveMessage = KeepaliveMessage::from_base(peek.metadata, payload);
                tracing::debug!("Got session_keepalive");
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

                tracing::debug!("Got notification");
                Ok(EventMessage::Notification(msg))
            }
            Err(e) => {
                tracing::error!("Failed to parse notification: {e}");
                Err(Error::SerdeError(e))
            }
        },
        _ => {
            tracing::warn!("Non-registered event was received");
            Ok(EventMessage::None)
        }
    }
}
