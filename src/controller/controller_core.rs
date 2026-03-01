use tokio_tungstenite::connect_async;

use super::helpers::handle_event;
use super::{
    Arc, BoxFuture, Client, DateTime, EventMessage, EventType, FutureExt, HashMap, MaybeTlsStream,
    Message, NotificationEvent, Result, RwLock, StreamExt, TcpStream, UserConfig, Utc,
    WebSocketStream,
};

type ArcCallbackMap<S, T> = Arc<RwLock<HashMap<S, T>>>;
type FutType = dyn Fn(NotificationEvent, DateTime<Utc>) -> BoxFuture<'static, ()> + Send + Sync;
#[rustfmt::skip]
type BoxedCallback =Box<dyn Fn(NotificationEvent, DateTime<Utc>) -> BoxFuture<'static, ()> + Send + Sync>;
pub struct TwitchController {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    session_id: Arc<RwLock<Option<String>>>,
    http_client: Arc<Client>,
    user_config: UserConfig,
    ntfy_callbacks: ArcCallbackMap<EventType, Box<FutType>>,
}

impl TwitchController {
    pub fn new(
        ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
        client: Client,
        user_config: UserConfig,
    ) -> Self {
        Self {
            ws,
            session_id: Arc::new(RwLock::new(None)),
            http_client: Arc::new(client),
            user_config,
            ntfy_callbacks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_callback<F, Fut>(&self, event_type: EventType, callback: F)
    where
        F: Fn(NotificationEvent, DateTime<Utc>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static, {
        // Code block starts here
        let boxed: BoxedCallback = Box::new(move |msg, dt| callback(msg, dt).boxed());
        let event_type_str: &String = &event_type.to_string();

        self.ntfy_callbacks.write().await.insert(event_type, boxed);
        tracing::info!("Registered callback for event type: {event_type_str}");
    }

    /// This function starts the main loop for keeping the connection.
    ///
    /// # Errors
    ///
    /// - Returns `serde_json::Error`, `anyhow::Error`, or `reqwest::Error` if
    ///   the `handle_event()` function fails
    pub async fn start(&mut self) -> Result<()> {
        let mut is_reconnect: bool = false;
        while let Some(msg) = self.ws.next().await {
            match msg {
                Ok(Message::Text(raw)) => {
                    let sid_clone: Arc<RwLock<Option<String>>> = Arc::clone(&self.session_id);
                    let http_client: Arc<Client> = Arc::clone(&self.http_client);

                    let msg: EventMessage = handle_event(
                        raw.as_str(),
                        sid_clone,
                        http_client,
                        &self.user_config,
                        is_reconnect,
                    )
                    .await?;

                    if let EventMessage::Reconnect(r) = &msg {
                        self.reconnect(r.payload.session.reconnect_url.clone()).await?;
                        is_reconnect = true;
                    } else {
                        if let EventMessage::Welcome(_) = &msg {
                            is_reconnect = false;
                        }

                        self.handle_message(msg).await;
                    }
                }
                Ok(Message::Close(frame)) => {
                    tracing::warn!("WebSocket closed: {frame:?}");
                }
                Ok(_) => (),
                Err(e) => tracing::error!("Error in main loop: {e}"),
            }
        }

        Ok(())
    }

    /// Reconnects with a new WebSocket connection, replacing the original
    /// dropped connection
    ///
    /// # Errors
    ///
    /// - Returns `tokio_tungstenite::error::Error` if failed to reconnect to
    ///   the new URL given by the Twitch API
    pub async fn reconnect(&mut self, reconnect_url: String) -> Result<()> {
        tracing::info!("Reconnecting to {reconnect_url}");
        let (ws_stream, _) = connect_async(reconnect_url).await?;
        self.ws = ws_stream;
        tracing::info!("Reconnected successfully");
        Ok(())
    }

    async fn handle_message(&self, msg: EventMessage) {
        match msg {
            EventMessage::Notification(ntf_msg) => {
                self.handle_notification_event(
                    ntf_msg.payload.event,
                    ntf_msg.metadata.message_timestamp,
                )
                .await;
            }
            EventMessage::Revocation(rev_msg) => {
                tracing::warn!(
                    "Subscription event revoked - type: {}, version: {}",
                    rev_msg.metadata.subscription_type,
                    rev_msg.metadata.subscription_version
                );
            }
            EventMessage::Welcome(welc_msg) => {
                let session_id: crate::prelude::welcome::WelcomeSession =
                    welc_msg.payload.session.unwrap();

                tracing::info!("Saved current session ID: {}", session_id.id);
            }
            _ => (),
        }
    }

    async fn handle_notification_event(&self, event: NotificationEvent, dt: DateTime<Utc>) {
        match event {
            NotificationEvent::ChannelChatMessage(ccm) => {
                if let Some(cb) = self.ntfy_callbacks.read().await.get(&EventType::ChatMessage) {
                    cb(NotificationEvent::ChannelChatMessage(ccm), dt).await;
                } else {
                    let msg: &str = "NotificationEvent was ChannelChatMessage, but there was no callback for it";
                    tracing::error!("{msg}");
                }
            }
            NotificationEvent::Other(_) => (),
        }
    }
}
