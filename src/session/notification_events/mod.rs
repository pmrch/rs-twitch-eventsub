mod channel_chat_message;

use channel_chat_message::ChannelChatMessage as CCM;

use crate::session::{Deserialize, MessageId, deserialize_message_id};

#[derive(Deserialize, Debug)]
pub enum NotificationEvent {
    ChannelChatMessage(Box<CCM>),
    Other(serde_json::Value),
}

pub use channel_chat_message::{ChannelChatMessage, ChatMessage};
