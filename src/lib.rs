pub mod controller;
pub mod error;
pub mod prelude;
pub mod session;
pub mod utils;

use prelude::{Result, TwitchController, UserConfig};
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
pub fn create_twitch_controller() -> Result<TwitchController> {
    dotenv::dotenv()?;
    crypto::ring::default_provider()
        .install_default()
        .expect("Failed to initialize TLS");

    let config: UserConfig = UserConfig::from_env()?;
    let controller: TwitchController = TwitchController::new(config);

    tracing::info!("Created controller, you can add handlers with `.register_callback()`");
    Ok(controller)
}
