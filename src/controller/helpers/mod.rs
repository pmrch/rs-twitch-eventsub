mod event_handler;
mod subscriber;

use std::sync::Arc;

pub use event_handler::{EventMessage, EventType, handle_event};
use reqwest::Client as RClient;
use serde_json::json;
pub use subscriber::subscribe_to_chat;

use crate::prelude::{Result, UserConfig};
