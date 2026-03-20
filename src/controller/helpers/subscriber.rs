use super::{
    Arc, ClientBuilder, EventType, HashMap, Mutex, Policy, RClient, Result, UserConfig, json,
};

struct SubscriptionData {
    sub_type: String,
    version: u32,
    condition: serde_json::Value,
}

impl SubscriptionData {
    pub fn new(sub_type: &str, version: u32, condition: serde_json::Value) -> Self {
        Self { sub_type: sub_type.into(), version, condition }
    }
}

pub struct Subscriber {
    client: Arc<RClient>,
    endpoint: Mutex<String>,
    config: UserConfig,
    session_id: Mutex<Option<String>>,
}

impl Subscriber {
    #[must_use]
    pub fn new(user_config: UserConfig) -> Self {
        let url: &str = "https://api.twitch.tv/helix/eventsub/subscriptions";
        let client: RClient = ClientBuilder::new()
            .redirect(Policy::none())
            .build()
            .expect("Failed to build reqwest client");

        Self {
            client: Arc::new(client),
            endpoint: Mutex::new(String::from(url)),
            config: user_config,
            session_id: Mutex::new(None),
        }
    }

    pub async fn update_endpoint(&self, new_endpoint: impl Into<String>) {
        *self.endpoint.lock().await = new_endpoint.into();
    }

    pub async fn set_session_id(&self, session_id: impl Into<String>) {
        *self.session_id.lock().await = Some(session_id.into());
    }

    /// This function handles subscribing to the specified event from Twitch API
    /// endpoint
    ///
    /// # Errors
    ///
    /// - Returns `reqwest::Error` if if there was an error while sending
    ///   request to API endpoint
    pub async fn subscribe(&self, sub_type: EventType) -> Result<()> {
        let sid_lock = self.session_id.lock().await;
        let session_id: String =
            sid_lock.clone().ok_or_else(|| anyhow::anyhow!("Session ID was not set!"))?;

        drop(sid_lock);
        let data: SubscriptionData = self
            .handle_subscription(&sub_type)
            .ok_or_else(|| anyhow::anyhow!("Invalid subscription type detected: {sub_type:#?}"))?;

        let body: serde_json::Value = json!({
            "type": data.sub_type,
            "version": format!("{}", data.version),
            "condition": data.condition,
            "transport": {
                "method": "websocket",
                "session_id": session_id
            }
        });

        println!("{}", &body.to_string());

        let response: reqwest::Response = self
            .client
            .post(&*self.endpoint.lock().await)
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
            tracing::error!("❌ Subscription failed: {error_text}, session_id: {session_id}");
        }

        Ok(())
    }

    fn build_condition(&self, event_type: &EventType) -> Result<serde_json::Value> {
        let mut map = HashMap::new();

        match event_type {
            EventType::ChatMessage => {
                map.insert("broadcaster_user_id", &self.config.broadcaster_id);
                map.insert("user_id", &self.config.user_id);
            }
            EventType::Bits => {
                map.insert("broadcaster_user_id", &self.config.broadcaster_id);
            }
            EventType::Subscription => (),
        }

        Ok(serde_json::to_value(map)?)
    }

    fn handle_subscription(&self, stype: &EventType) -> Option<SubscriptionData> {
        let condition: serde_json::Value = self.build_condition(stype).ok()?;
        match stype {
            EventType::ChatMessage => {
                Some(SubscriptionData::new("channel.chat.message", 1, condition))
            }
            EventType::Bits => Some(SubscriptionData::new("channel.cheer", 1, condition)),
            EventType::Subscription => None,
        }
    }
}
