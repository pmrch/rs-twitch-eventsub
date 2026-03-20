use tokio_tungstenite::connect_async;

use super::helpers::{handle_event, handle_ws_error, hne, Subscriber};
use super::{
    Arc, ArcCallbackMap, BoxedCallback, Client, ClientBuilder, DateTime, ErrorAction, EventMessage, EventType,
    FutType, FutureExt, HashMap, MaybeTlsStream, Message, NotificationEvent, Result, RwLock,
    StreamExt, TcpStream, UserConfig, Utc, WebSocketStream, Policy
};

struct Endpoints {
    http: String,
    ws: String
}

impl Endpoints {
    #[must_use]
    pub fn new(http_endpoint: impl Into<String>, ws_endpoint: impl Into<String>) -> Self {
        Self {
            http: http_endpoint.into(),
            ws: ws_endpoint.into()
        }
    }
}

pub struct TwitchController {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    session_id: Arc<RwLock<Option<String>>>,
    http_client: Arc<Client>,
    user_config: UserConfig,
    ntfy_callbacks: ArcCallbackMap<EventType, Box<FutType>>,
    endpoints: Endpoints
}

impl TwitchController {
    pub fn new(
        ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
        user_config: UserConfig,
        ws_endpoint: String,
    ) -> Result<Self> {
        let endpoints: Endpoints = Endpoints::new(
            "https://api.twitch.tv/helix/eventsub/subscriptions", 
            ws_endpoint
        );

        let client: Client = ClientBuilder::new().redirect(Policy::none()).build()?;
        Ok(Self {
            ws,
            session_id: Arc::new(RwLock::new(None)),
            http_client: Arc::new(client),
            user_config,
            ntfy_callbacks: Arc::new(RwLock::new(HashMap::new())),
            endpoints
        })
    }

    pub fn set_dev_mode(&mut self, http_endpoint: &str, ws_endpoint: String) {
        self.endpoints.http = http_endpoint.to_string();
        self.endpoints.ws = ws_endpoint;
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
                        &self.endpoints.http,
                    )
                    .await?;

                    if let EventMessage::Reconnect(r) = &msg {
                        self.endpoints.ws.clone_from(&r.payload.session.reconnect_url);
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
                Err(e) => match handle_ws_error(e) {
                    ErrorAction::Skip => (),
                    ErrorAction::Reconnect => self.reconnect(self.endpoints.ws.clone()).await?,
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
    pub async fn reconnect(&mut self, reconnect_url: String) -> Result<()> {
        const MAX_RETRIES: u32 = 5;
        let mut attempt = 0;

        loop {
            attempt += 1;
            tracing::info!("Reconnect attempt {attempt}/{MAX_RETRIES} to {reconnect_url}");

            match connect_async(&reconnect_url).await {
                Ok((ws_stream, _)) => {
                    self.ws = ws_stream;
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
