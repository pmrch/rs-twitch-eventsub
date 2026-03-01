use super::{Deserialize, MessageId, MessageType};

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Badge {
    /// An ID that identifies this set of chat badges. For example, Bits or
    /// Subscriber.
    pub set_id: String,

    /// An ID that identifies this version of the badge. The ID can be any
    /// value. For example, for Bits, the ID is the Bits tier level, but for
    /// World of Warcraft, it could be Alliance or Horde.
    pub id: String,

    /// Contains metadata related to the chat badges in the badges tag.
    /// Currently, this tag contains metadata only for subscriber badges, to
    /// indicate the number of months the user has been a subscriber.
    pub info: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Cheermote {
    /// The full Cheermote string is the concatenation of {prefix} + {number of
    /// Bits}. For example, a full Cheermote string is Cheer100.
    pub prefix: String,

    /// Amount of bits cheered
    pub bits: u32,

    /// The tier level of the cheermote
    pub tier: u32,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Emote {
    /// An ID that uniquely identifies this emote.
    pub id: String,

    /// An ID that identifies the emote set that the emote belongs to.
    #[serde(rename = "emote_set_id")]
    pub set_id: String,

    /// The ID of the broadcaster who owns the emote.
    pub owner_id: String,

    /// The formats that the emote is available in. This can be:
    ///   - animated - An animated GIF is available for this emote.
    ///   - static - A static PNG file is available for this emote.
    ///
    /// Either one or both of them will be present in the `Vec<T>`
    pub format: Vec<String>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Mention {
    /// The user ID of the mentioned user.
    #[serde(rename = "user_id")]
    pub id: String,

    /// The user name of the mentioned user.
    #[serde(rename = "user_name")]
    pub name: String,

    /// The user login of the mentioned user.
    #[serde(rename = "user_login")]
    pub login: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Fragment {
    #[serde(rename = "type")]
    /// Possible values: text, cheermote, emote, mention
    pub ftype: String,

    /// Message text in fragment.
    pub text: String,

    /// Optional. Metadata pertaining to the cheermote.
    pub cheermote: Option<Cheermote>,

    /// Optional. Metadata pertaining to the emote.
    pub emote: Option<Emote>,

    /// Optional. Metadata pertaining to the mention.
    pub mention: Option<Mention>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct ChatMessage {
    /// The chat message in plain text.
    pub text: String,

    /// Ordered list of chat message fragments.
    pub fragments: Vec<Fragment>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Reply {
    /// An ID that uniquely identifies the parent message that this message
    /// is replying to.
    pub parent_message_id: String,

    /// The message body of the parent message.
    pub parent_message_body: String,

    /// User ID of the sender of the parent message.
    pub parent_user_id: String,

    /// User name of the sender of the parent message.
    pub parent_user_name: String,

    /// User login of the sender of the parent message.
    pub parent_user_login: String,

    /// An ID that identifies the parent message of the reply thread.
    pub thread_message_id: String,

    /// User ID of the sender of the thread’s parent message.
    pub thread_user_id: String,

    /// User name of the sender of the thread’s parent message.
    pub thread_user_name: String,

    /// User login of the sender of the thread’s parent message.
    pub thread_user_login: String,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct ChannelChatMessage {
    /// The broadcaster user ID.
    pub broadcaster_user_id: String,

    // The broadcaster display name.
    pub broadcaster_user_name: String,

    /// The broadcaster login.
    pub broadcaster_user_login: String,

    /// The user ID of the user that sent the message.
    pub chatter_user_id: String,

    /// The user name of the user that sent the message.
    pub chatter_user_name: String,

    /// The user login of the user that sent the message.
    pub chatter_user_login: String,

    /// A UUID that identifies the message.
    #[serde(deserialize_with = "super::deserialize_message_id")]
    pub message_id: MessageId,

    /// The structured chat message.
    pub message: ChatMessage,

    /// The type of message fragment. Possible values:
    ///   - `text`
    ///   - `channel_points_highlighted`
    ///   - `channel_points_sub_only`
    ///   - `user_intro`
    ///   - `power_ups_message_effect`
    ///   - `power_ups_gigantified_emote`
    pub message_type: MessageType,

    /// List of chat badges.
    pub badges: Vec<Badge>,

    /// Optional. Metadata if this message is a reply.
    pub reply: Option<Reply>,

    /// Optional. The ID of a channel points custom reward that was redeemed.
    pub channel_points_custom_reward_id: Option<String>,

    /// Optional. The broadcaster user ID of the channel the message was sent
    /// from. Is null when the message happens in the same channel as the
    /// broadcaster.
    pub source_broadcaster_user_id: Option<String>,

    /// Optional. The user name of the broadcaster of the channel the message
    /// was sent from. Is null when the message happens in the same channel
    /// as the broadcaster.
    pub source_broadcaster_user_name: Option<String>,

    /// Optional. The login of the broadcaster of the channel the message was
    /// sent from. Is null when the message happens in the same channel as
    /// the broadcaster.
    pub source_broadcaster_user_login: Option<String>,

    /// Optional. The UUID that identifies the source message from the channel
    /// the message was sent from.
    /// Is null when the message happens in the same channel as the broadcaster.
    pub source_message_id: Option<String>,

    /// Optional. The list of chat badges for the chatter in the channel the
    /// message was sent from. Is null when the message happens in the same
    /// channel as the broadcaster.
    pub source_badges: Option<Vec<Badge>>,

    /// Optional. Determines if a message delivered during a shared chat session
    /// is only sent to the source channel.
    /// Has no effect if the message is not sent during a shared chat session.
    pub is_source_only: Option<bool>,
}
