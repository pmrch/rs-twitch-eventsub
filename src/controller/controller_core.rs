use super::helpers::handle_event;
use super::{
    Arc, ChatMessage, Client, EventMessage, MaybeTlsStream, Message, NotificationEvent, Result,
    RwLock, StreamExt, TcpStream, UserConfig, WebSocketStream,
};

pub struct TwitchController<Fut: std::future::Future<Output = ()> + Send + 'static> {
    ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
    session_id: Arc<RwLock<Option<String>>>,
    http_client: Arc<Client>,
    user_config: UserConfig,
    chat_message_callback: Box<dyn Fn(ChatMessage, String) -> Fut + Send + Sync>,
}

impl<Fut> TwitchController<Fut>
where
    Fut: Future<Output = ()> + Send + 'static,
{
    pub fn new(
        ws: WebSocketStream<MaybeTlsStream<TcpStream>>,
        client: Client,
        user_config: UserConfig,
        callback: impl Fn(ChatMessage, String) -> Fut + Send + Sync + 'static,
    ) -> Self {
        Self {
            ws,
            session_id: Arc::new(RwLock::new(None)),
            http_client: Arc::new(client),
            user_config,
            chat_message_callback: Box::new(callback),
        }
    }

    /// This function starts the main loop for keeping the connection.
    ///
    /// # Errors
    ///
    /// - Returns `serde_json::Error`, `anyhow::Error`, or `reqwest::Error` if
    ///   the `handle_event()` function fails
    pub async fn start(&mut self) -> Result<()> {
        while let Some(msg) = self.ws.next().await {
            match msg {
                Ok(Message::Text(raw)) => {
                    let sid_clone: Arc<RwLock<Option<String>>> = Arc::clone(&self.session_id);
                    let http_client: Arc<Client> = Arc::clone(&self.http_client);
                    let msg: EventMessage =
                        handle_event(raw.as_str(), sid_clone, http_client, &self.user_config)
                            .await?;

                    self.handle_message(msg).await;
                }
                Ok(Message::Close(frame)) => {
                    tracing::warn!("WebSocket closed: {frame:?}");
                    break;
                }
                Ok(_) => (),
                Err(e) => tracing::error!("Error in main loop: {e}"),
            }
        }

        Ok(())
    }

    async fn handle_message(&self, msg: EventMessage) {
        match msg {
            EventMessage::Notification(ntf_msg) => {
                self.handle_notification_event(ntf_msg.payload.event).await;
            }
            EventMessage::Welcome(welc_msg) => {
                let session_id: crate::prelude::welcome::WelcomeSession =
                    welc_msg.payload.session.unwrap();

                tracing::info!("Saved current session ID: {}", session_id.id);
            }
            _ => (),
        }
    }

    async fn handle_notification_event(&self, event: NotificationEvent) {
        match event {
            NotificationEvent::ChannelChatMessage(ccm) => {
                (self.chat_message_callback)(ccm.message, ccm.chatter_user_name).await;
            }
            NotificationEvent::Other(_) => (),
        }
    }
}
