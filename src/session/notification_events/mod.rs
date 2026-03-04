mod ccb;
mod ccm;

pub use ccb::ChannelCheer as CC;
pub use ccm::{ChannelChatMessage as CCM, ChatMessage};

use super::Deserialize;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum NotificationEvent {
    ChannelChatMessage(Box<CCM>),
    ChannelCheer(Box<CC>),
    Other(serde_json::Value),
}
