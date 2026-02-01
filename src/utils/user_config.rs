use crate::prelude::Result;

#[derive(Debug, Default)]
pub struct UserConfig {
    pub client_id: String,
    pub user_token: String,
    pub broadcaster_id: String,
    pub user_id: String,
}

impl UserConfig {
    /// This function lets you create a default instance of `UserConfig`
    /// without specifying fields. You must have them in `.env` for this
    /// function to work.
    ///
    /// # Errors
    ///
    /// - Returns `std::env::VarError` if the default environment variables
    ///   for `UserConfig` are not found in `.env`
    pub fn from_env() -> Result<Self> {
        let client_id: String = std::env::var("TWITCH_CLIENT_ID")?;
        let user_token: String = std::env::var("TWITCH_TOKEN")?;
        let broadcaster_id: String = std::env::var("BROADCASTER_ID")?;
        let user_id: String = std::env::var("USER_ID")?;

        Ok(Self { client_id, user_token, broadcaster_id, user_id })
    }

    /// This function lets you create a default instance of `UserConfig`
    /// with provided broadcaster ID, filling other fields from `.env`
    ///
    /// # Errors
    ///
    /// - Returns `std::env::VarError` if the default environment variables
    ///   for `UserConfig` are not found in `.env`
    pub fn with_broadcaster_id(self, id: String) -> Result<Self> {
        let mut initial_config: Self = Self::from_env()?;
        initial_config.broadcaster_id = id;
        Ok(initial_config)
    }

    pub fn change_broadcaster_id(&mut self, id: String) { self.broadcaster_id = id; }
}
