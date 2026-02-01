use super::{Arc, RClient, Result, UserConfig, json};

/// This function handles subscribing to the `channel.chat.message`
/// event from Twitch API endpoint
///
/// # Errors
///
/// - Returns `reqwest::Error` if if there was an error while sending
///   request to API endpoint
///
pub async fn subscribe_to_chat(
    client: Arc<RClient>,
    session_id: &str,
    config: &UserConfig,
) -> Result<()> {
    let body: serde_json::Value = json!({
        "type": "channel.chat.message",
        "version": "1",
        "condition": {
            "broadcaster_user_id": config.broadcaster_id,
            "user_id": config.user_id
        },
        "transport": {
            "method": "websocket",
            "session_id": session_id
        }
    });

    let response: reqwest::Response = client
        .post("https://api.twitch.tv/helix/eventsub/subscriptions")
        .header("Authorization", format!("Bearer {}", config.user_token))
        .header("Client-Id", &config.client_id)
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
