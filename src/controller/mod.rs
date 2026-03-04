mod controller_core;
mod helpers;

use std::collections::HashMap;
use std::sync::Arc;

pub use controller_core::TwitchController;
use futures::FutureExt;
use futures::future::BoxFuture;
pub use helpers::{EventMessage, EventType};
use tokio::sync::RwLock;

use crate::prelude::keepalive::{KeepaliveMessage, KeepalivePayload};
use crate::prelude::notification::{NotificationMessage, NotificationPayload};
use crate::prelude::reconnect::{ReconnectMessage, ReconnectPayload};
use crate::prelude::revocation::{RevocationMessage, RevocationPayload};
use crate::prelude::welcome::{WelcomeMessage, WelcomePayload};
use crate::prelude::*;
pub use crate::session::ChatMessage;

pub type ArcCallbackMap<S, T> = Arc<RwLock<HashMap<S, T>>>;
pub type FutType = dyn Fn(NotificationEvent, DateTime<Utc>) -> BoxFuture<'static, ()> + Send + Sync;

pub type BoxedCallback =
    Box<dyn Fn(NotificationEvent, DateTime<Utc>) -> BoxFuture<'static, ()> + Send + Sync>;
