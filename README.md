# rs-twitch-eventsub

A lightweight async library for handling Twitch EventSub WebSocket events — only covering the specific event types I currently need.  
It's built on top of **tokio**, **reqwest**, and **tokio-tungstenite**, with a focus on being simple, transparent, and reliable.

---

## ⚡ Overview

`rs-twitch-eventsub` sets up a Twitch EventSub WebSocket session, automatically subscribes to events, and handles a small, 
carefully chosen subset of events:

- ✅ `session_welcome`
- ✅ `session_keepalive`
- ✅ `session_reconnect`
- ✅ `notification` (with `channel.chat.message` and `channel.cheer`)
- ✅ `revocation` (logged as a warning)
- ⚠️ Other events are recognized but ignored with a warning.

This library is **not** a full Twitch SDK — it's meant for small integrations, personal bots, and experiments where you only need core 
EventSub behavior and want full control of the flow. However in the future I might extend.

---

## 🔧 Example

```rust
use twitch_eventsub::create_twitch_controller;
use twitch_eventsub::prelude::{EventType, TwitchController};
use twitch_eventsub::session::NotificationEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure tracing if you want logs:
    // let subscriber = tracing_subscriber::FmtSubscriber::builder()
    //     .with_max_level(tracing::Level::INFO)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber)?;

    // Or use the library-provided logger setup that logs to file in `logs/` on your
    // project root, and also logs to console
    // setup_logger(None)?; // None defaults to "info" level
    // setup_logger(Some("debug"))?; // or specify a level

    let mut twitch_controller: TwitchController = create_twitch_controller(None).await?;

    // Optional: use local Twitch CLI WebSocket server for dev/testing
    // For this purpose I added `set_dev_mode()`
    // let mut twitch_controller = create_twitch_controller(Some("ws://127.0.0.1:8080/ws")).await?;
    // twitch_controller.set_dev_mode("http://127.0.0.1:8080/eventsub/subscriptions");

    twitch_controller
        .register_callback(EventType::ChatMessage, |event, _dt| async move {
            if let NotificationEvent::ChannelChatMessage(ccm) = event {
                println!("{}: {}", ccm.chatter_user_name, ccm.message.text);
            }
        }).await;

    twitch_controller
        .register_callback(EventType::Bits, |event, _dt| async move {
            if let NotificationEvent::ChannelCheer(ccb) = event {
                println!("{} cheered {} bits!", ccb.user_name.as_deref().unwrap_or("anonymous"), ccb.bits);
            }
        }).await;

    twitch_controller.start().await?;
    Ok(())
}
```

---

## 📡 Handled Events

| Event Type | Description | Status |
|-------------|--------------|--------|
| `session_welcome` | Saves session ID and subscribes to events | ✅ |
| `session_keepalive` | Keeps the connection alive | ✅ |
| `session_reconnect` | Transparently reconnects to the new URL provided by Twitch | ✅ |
| `notification` | Handles `channel.chat.message` and `channel.cheer` | ✅ |
| `revocation` | Logs subscription revocation as a warning | ✅ |
| *other events* | Logged but ignored | ⚠️ ignored |

---

## 🧰 Requirements

- Rust 1.75 or newer  
- A Twitch Application (Client ID & Secret)  
- `tokio` runtime (full features)  

Example `.env` file:

```
TWITCH_CLIENT_ID=your_client_id
TWITCH_CLIENT_SECRET=your_client_secret
USER_ID=your_bot_user_id
BROADCASTER_ID=target_channel_id
TWITCH_USER_TOKEN=your_user_access_token
```

> ⚠️ `channel.cheer` requires a **broadcaster** user access token with the `bits:read` scope — the bot's token is not sufficient.
> For local testing, use the [Twitch CLI](https://dev.twitch.tv/docs/cli/) mock WebSocket server with `set_dev_mode()`.

---

## 📥 Installation
### GitHub
Add the following lines to your `Cargo.toml` in your project root:

```toml
[dependencies]
twitch-eventsub = { git = "https://github.com/pmrch/rs-twitch-eventsub", branch = "main" }
```

### crates.io (for later)
Once the API is stable and documented, I might publish to crates.io.

---

## 🪶 Tracing / Logging

`twitch-eventsub` never sets a global tracing subscriber by itself (that would violate library crate rules).  
If you want logs with tracing, configure a global one yourself, or use the provided `setup_logger()` exposed through `prelude.rs`:

```rust
use tracing_subscriber::FmtSubscriber;

let subscriber = FmtSubscriber::builder()
    .with_max_level(tracing::Level::DEBUG)
    .finish();
tracing::subscriber::set_global_default(subscriber)?;
```

Or use the predefined logger that logs to file in `logs/` and to console:
```rust
use twitch_eventsub::prelude::setup_logger;

setup_logger(None)?;          // defaults to "info"
setup_logger(Some("debug"))?; // or specify a level
setup_logger(Some("trace"))?; // for full granularity
```

If you don't set up any logger, it'll stay silent.

---

## 🧪 Local Development / Testing

To test without connecting to real Twitch, use the [Twitch CLI](https://dev.twitch.tv/docs/cli/) mock WebSocket server:

```bash
twitch event websocket start-server
```

Then point your controller at the local server:

```rust
let mut controller = create_twitch_controller(Some("ws://127.0.0.1:8080/ws")).await?;
controller.set_dev_mode("http://127.0.0.1:8080/eventsub/subscriptions");
```

Trigger mock events in a separate terminal:

```bash
twitch event trigger cheer -F ws://127.0.0.1:8080/ws --transport websocket
```

---

## 🤝 Contributing

Pull requests are **welcome**, but **I'll only merge them if they align with the crate's current goals and style.**  
This isn't a general-purpose Twitch library — it's a focused one.  
That said, if you've got a good addition or cleanup that fits well, I'll happily review it.
Moreover you are free to fork this repository and maintain your own version.

---

## 📜 License

MIT © 2026  
Free to use, modify, and adapt with attribution.

---

## 💬 Author

Created by **pmrch** — just a self-learnt developer in high-school. Claude.ai also assisted in the WebSocket
setup with tokio-tungstenite. This project is part of a larger personal goal.
