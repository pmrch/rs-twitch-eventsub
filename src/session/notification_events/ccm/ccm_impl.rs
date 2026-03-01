use super::{MessageId, MessageType};

impl super::MessageType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::UserIntro => "user_intro",
            Self::PowerUpsMessageEffect => "power_ups_message_effect",
            Self::PowerUpsGigantifiedEmote => "power_ups_gigantified_emote",
            Self::ChannelPointsSubOnly => "channel_points_sub_only",
            Self::ChannelPointsHighlighted => "channel_points_highlighted",
        }
    }

    pub fn from_str_snake_case(s: &str) -> anyhow::Result<Self> {
        match s {
            "text" => Ok(Self::Text),
            "user_intro" => Ok(Self::UserIntro),
            "power_ups_message_effect" => Ok(Self::PowerUpsMessageEffect),
            "power_ups_gigantified_emote" => Ok(Self::PowerUpsGigantifiedEmote),
            "channel_points_sub_only" => Ok(Self::ChannelPointsSubOnly),
            "channel_points_highlighted" => Ok(Self::ChannelPointsHighlighted),
            _ => Err(anyhow::anyhow!("Invalid option provided to convert from")),
        }
    }

    pub fn from_str_camel_case(s: &str) -> anyhow::Result<Self> {
        match s {
            "text" => Ok(Self::Text),
            "userIntro" => Ok(Self::UserIntro),
            "powerUpsMessageEffect" => Ok(Self::PowerUpsMessageEffect),
            "powerUpsGigantifiedEmote" => Ok(Self::PowerUpsGigantifiedEmote),
            "channelPointsSubOnly" => Ok(Self::ChannelPointsSubOnly),
            "channelPointsHighlighted" => Ok(Self::ChannelPointsHighlighted),
            _ => Err(anyhow::anyhow!("Invalid camelCase option: {s}")),
        }
    }

    pub fn from_str_pascal_case(s: &str) -> anyhow::Result<Self> {
        match s {
            "Text" => Ok(Self::Text),
            "UserIntro" => Ok(Self::UserIntro),
            "PowerUpsMessageEffect" => Ok(Self::PowerUpsMessageEffect),
            "PowerUpsGigantifiedEmote" => Ok(Self::PowerUpsGigantifiedEmote),
            "ChannelPointsSubOnly" => Ok(Self::ChannelPointsSubOnly),
            "ChannelPointsHighlighted" => Ok(Self::ChannelPointsHighlighted),
            _ => Err(anyhow::anyhow!("Invalid PascalCase option: {s}")),
        }
    }
}

impl std::fmt::Display for super::MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = self.as_str();
        write!(f, "{s}")
    }
}

impl TryFrom<&str> for MessageType {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let universal: String = s.to_lowercase().replace(['_', '-'], "");

        match universal.as_str() {
            "text" => Ok(Self::Text),
            "userintro" => Ok(Self::UserIntro),
            "powerupsmessageeffect" => Ok(Self::PowerUpsMessageEffect),
            "powerupsgigaantifiedemote" => Ok(Self::PowerUpsGigantifiedEmote),
            "channelpointssubonly" => Ok(Self::ChannelPointsSubOnly),
            "channelpointshighlighted" => Ok(Self::ChannelPointsHighlighted),
            _ => Err(anyhow::anyhow!("Invalid option: {s}!")),
        }
    }
}

impl TryFrom<String> for MessageType {
    type Error = anyhow::Error;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let universal: String = s.to_lowercase().replace(['_', '-'], "");

        match universal.as_str() {
            "text" => Ok(Self::Text),
            "userintro" => Ok(Self::UserIntro),
            "powerupsmessageeffect" => Ok(Self::PowerUpsMessageEffect),
            "powerupsgigaantifiedemote" => Ok(Self::PowerUpsGigantifiedEmote),
            "channelpointssubonly" => Ok(Self::ChannelPointsSubOnly),
            "channelpointshighlighted" => Ok(Self::ChannelPointsHighlighted),
            _ => Err(anyhow::anyhow!("Invalid option: {s}!")),
        }
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String = match self {
            Self::StringId(suid) => suid.clone(),
            Self::UuidId(uuid) => uuid.to_string(),
        };

        write!(f, "{s}")
    }
}
