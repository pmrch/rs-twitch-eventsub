use super::{Arc, ClientBuilder, Policy, RClient, Result, RwLock, UserConfig, json};

pub struct Subscriber {
    client: Arc<RClient>,
    endpoint: String,
    config: UserConfig,
    session_id: RwLock<Option<String>>
}

impl Subscriber {
    #[must_use]
    pub fn new(user_config: UserConfig) -> Self {
        let client: RClient =  ClientBuilder::new()
            .redirect(Policy::none())
            .build()
            .expect("Failed to build reqwest client");

        Self { 
            client: Arc::new(client),
            endpoint: String::from("https://api.twitch.tv/helix/eventsub/subscriptions"),
            config: user_config,
            session_id: RwLock::new(None)
        }
    }

    pub fn update_endpoint(&mut self, new_endpoint: impl Into<String>) {
        self.endpoint = new_endpoint.into();
    }

    pub async fn set_session_id(&self, session_id: impl Into<String>) {
        *self.session_id.write().await = Some(session_id.into());
    }

    /// This function handles subscribing to the specified event from Twitch API 
    /// endpoint
    ///
    /// # Errors
    ///
    /// - Returns `reqwest::Error` if if there was an error while sending request to
    ///   API endpoint
    pub async fn subscribe_to_event(&self) -> Result<()> {
        let sid_lock = self.session_id.read().await;
        let session_id: &String = sid_lock
            .as_ref()
            .ok_or(anyhow::anyhow!("Session ID was not set!"))?;

        let body: serde_json::Value = json!({
            "type": "channel.chat.message",
            "version": "1",
            "condition": {
                "broadcaster_user_id": &self.config.broadcaster_id,
                "user_id": &self.config.user_id
            },
            "transport": {
                "method": "websocket",
                "session_id": session_id
            }
        });

        let response: reqwest::Response = self.client
            .post(&self.endpoint)
            .header("Authorization", format!("Bearer {}", self.config.user_token))
            .header("Client-Id", &self.config.client_id)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if response.status().is_success() {
            tracing::info!("✅ Subscribed to channel.chat.message!");
        } else {
            let error_text: String = response.text().await?;
            tracing::error!("❌ Subscription failed: {error_text}");
        }

        Ok(())
    }
}