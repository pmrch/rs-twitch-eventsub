use super::Deserialize;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct ChannelCheer {
    /// Whether the user cheered anonymously or not.
    pub is_anonymous: bool,

    /// The user ID for the user who cheered on the specified channel. This is
    /// null if `is_anonymous` is true.
    pub user_id: Option<String>,

    /// The user login for the user who cheered on the specified channel. This
    /// is null if `is_anonymous` is true.
    pub user_login: Option<String>,

    /// The user display name for the user who cheered on the specified channel.
    /// This is null if `is_anonymous` is true.
    pub user_name: Option<String>,

    /// The requested broadcaster ID.
    pub broadcaster_user_id: String,

    /// The requested broadcaster login.
    pub broadcaster_user_login: String,

    /// The requested broadcaster display name.
    pub broadcaster_user_name: String,

    /// The requested broadcaster display name.
    pub message: String,

    /// The number of Bits cheered.
    pub bits: u32,
}
