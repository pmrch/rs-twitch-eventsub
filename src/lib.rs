pub mod controller;
pub mod error;
pub mod prelude;
pub mod session;
pub mod utils;

use prelude::{Result, TwitchController, Url, UserConfig, connect_async};
use rustls::crypto;

/// This function starts the main loop for `TwitchController`
///
/// # Errors
///
/// - Returns `tokio_tungstenite::tungstenite::Error` if any sort of WebSocket
///   call fails
///
/// # Panics
///
/// - Panics if TLS initialization fails
pub async fn create_twitch_controller(url: Option<&str>) -> Result<TwitchController> {
    dotenv::dotenv()?;
    crypto::ring::default_provider()
        .install_default()
        .expect("Failed to initialize TLS");

    let url: Url = if let Some(u) = url {
        Url::parse(u)?
    } else {
        Url::parse("wss://eventsub.wss.twitch.tv/ws")?
    };

    let (ws_stream, _) = connect_async(url.to_string()).await?;
    let config: UserConfig = UserConfig::from_env()?;
    let controller: TwitchController = TwitchController::new(ws_stream, config, url.to_string());

    tracing::info!("Created controller, you can add handlers with `.register_callback()`");
    Ok(controller)
}
