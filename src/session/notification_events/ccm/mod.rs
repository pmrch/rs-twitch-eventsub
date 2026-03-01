mod ccm_impl;
mod channel_chat_message;

use channel_chat_message::ChannelChatMessage as CCM;

use crate::prelude::{Deserialize, MessageId, deserialize_message_id};

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum NotificationEvent {
    ChannelChatMessage(Box<CCM>),
    Other(serde_json::Value),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Text,
    ChannelPointsHighlighted,
    ChannelPointsSubOnly,
    UserIntro,
    PowerUpsMessageEffect,
    PowerUpsGigantifiedEmote,
}

pub use channel_chat_message::{ChannelChatMessage, ChatMessage};
