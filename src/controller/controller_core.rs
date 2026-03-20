use tokio_tungstenite::connect_async;

use super::helpers::{Subscriber, handle_event, handle_ws_error, hne};
use super::{
    Arc, ArcCallbackMap, BoxedCallback, DateTime, ErrorAction, EventMessage, EventType, FutType,
    FutureExt, HashMap, MaybeTlsStream, Message, NotificationEvent, Result, RwLock, StreamExt,
    TcpStream, UserConfig, Utc, WebSocketStream,
};

pub struct TwitchController {
    ws: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    ws_endpoint: String,
    ntfy_callbacks: ArcCallbackMap<EventType, Box<FutType>>,
    subscriber: Arc<Subscriber>,
}

impl TwitchController {
    #[must_use]
    pub fn new(user_config: UserConfig) -> Self {
        Self {
            ws: None,
            ws_endpoint: String::from("wss://eventsub.wss.twitch.tv/ws"),
            ntfy_callbacks: Arc::new(RwLock::new(HashMap::new())),
            subscriber: Arc::new(Subscriber::new(user_config)),
        }
    }

    pub async fn set_dev_mode(&mut self, http_endpoint: &str, ws_endpoint: String) {
        self.subscriber.update_endpoint(http_endpoint).await;
        self.ws_endpoint = ws_endpoint;
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
        if self.ws.is_none() {
            let (ws_stream, _) = connect_async(&self.ws_endpoint).await?;
            self.ws = Some(ws_stream);
        }

        let mut ws: WebSocketStream<MaybeTlsStream<TcpStream>> = self
            .ws
            .take()
            .ok_or_else(|| anyhow::anyhow!("Failed to take ownership of WebSocket"))?;

        let mut is_reconnect: bool = false;
        while let Some(msg) = ws.next().await {
            match msg {
                Ok(Message::Text(raw)) => {
                    let msg: EventMessage =
                        handle_event(&raw, is_reconnect, Arc::clone(&self.subscriber)).await?;

                    if let EventMessage::Reconnect(r) = &msg {
                        self.ws_endpoint.clone_from(&r.payload.session.reconnect_url);
                        self.reconnect().await?;
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
                Err(e) => match handle_ws_error(e) {
                    ErrorAction::Skip => (),
                    ErrorAction::Reconnect => self.reconnect().await?,
                    ErrorAction::Fatal(reason) => return Err(reason),
                },
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
    pub async fn reconnect(&mut self) -> Result<()> {
        const MAX_RETRIES: u32 = 5;
        let mut attempt: u32 = 0;
        let reconnect_url: &String = &self.ws_endpoint;

        loop {
            attempt += 1;
            tracing::info!("Reconnect attempt {attempt}/{MAX_RETRIES} to {reconnect_url}");

            match connect_async(reconnect_url).await {
                Ok((ws_stream, _)) => {
                    self.ws = Some(ws_stream);
                    tracing::info!("Reconnected successfully on attempt {attempt}");
                    return Ok(());
                }
                Err(e) => {
                    if attempt >= MAX_RETRIES {
                        tracing::error!("All {MAX_RETRIES} reconnect attempts failed, giving up");
                        return Err(e.into());
                    }

                    let delay = std::time::Duration::from_secs(2u64.pow(attempt));
                    tracing::warn!(
                        "Reconnect attempt {attempt} failed: {e}, retrying in {}s",
                        delay.as_secs()
                    );
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    async fn handle_message(&self, msg: EventMessage) {
        match msg {
            EventMessage::Notification(ntf_msg) => {
                let cloned_callbacks = Arc::clone(&self.ntfy_callbacks);
                hne(ntf_msg.payload.event, ntf_msg.metadata.message_timestamp, cloned_callbacks)
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
}
