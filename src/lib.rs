pub mod controller;
pub mod error;
pub mod prelude;
pub mod session;
pub mod utils;

use controller::TwitchController;
use prelude::*;

/// This function starts the main loop for `TwitchController`
///
/// # Errors
///
/// - Returns `tokio_tungstenite::tungstenite::Error` if any sort of WebSocket
///   call fails
pub async fn run_twitch_controller<F, Fut>(message_handler: F) -> Result<()>
where
    F: Fn(ChatMessage, String) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = ()> + Send + 'static, {
    let url: Url = Url::parse("wss://eventsub.wss.twitch.tv/ws")?;
    let (ws_stream, _) = connect_async(url.to_string()).await?;

    let https_client: Client = ClientBuilder::new().redirect(Policy::none()).build()?;
    let config: UserConfig = UserConfig::from_env()?;

    let mut controller: TwitchController<_> =
        TwitchController::new(ws_stream, https_client, config, message_handler);

    controller.start().await?;
    Ok(())
}
