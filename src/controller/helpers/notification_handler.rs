use super::{ArcCallbackMap, DateTime, EventType, FutType, NotificationEvent, Utc};

type Callbacks = ArcCallbackMap<EventType, Box<FutType>>;

#[rustfmt::skip]
pub async fn handle_notification_event(
    event: NotificationEvent,
    dt: DateTime<Utc>,
    callbacks: Callbacks,
) {
    match event {
        NotificationEvent::ChannelChatMessage(ccm) => {
            if let Some(cb) = callbacks.read().await.get(&EventType::ChatMessage) {
                cb(NotificationEvent::ChannelChatMessage(ccm), dt).await;
                return;
            }
            tracing::debug!("NotificationEvent was ChannelChatMessage, but there was no callback for it!");
        }
        NotificationEvent::ChannelCheer(ccb) => {
            if let Some(cb) = callbacks.read().await.get(&EventType::Bits) {
                cb(NotificationEvent::ChannelCheer(ccb), dt).await;
                tracing::debug!("Found a Cheer!");
                return;
            }
            tracing::debug!("NotificationEvent was ChannelCheer, but there was no callback for it!");
        }
        NotificationEvent::Other(_) => (),
    }
}
