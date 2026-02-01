# rs-twitch-eventsub

A lightweight async library for handling Twitch EventSub WebSocket events — only covering the specific event types I currently need.  
It’s built on top of **tokio**, **reqwest**, and **tokio-tungstenite**, with a focus on being simple, transparent, and reliable.

---

## ⚡ Overview

`rs-twitch-eventsub` sets up a Twitch EventSub WebSocket session, automatically subscribes to chat messages, and handles a small, 
carefully chosen subset of events:

- ✅ `session_welcome`
- ✅ `session_keepalive`
- ✅ `notification` (with `channel.chat.message`)
- ⚠️ Other events are recognized but ignored with a warning.

This library is **not** a full Twitch SDK — it’s meant for small integrations, personal bots, and experiments where you only need core 
EventSub behavior and want full control of the flow. However in the future I might extend.

---

## 🔧 Example

```rust
use twitch_eventsub::run_twitch_controller;
use twitch_eventsub::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Configure tracing if you want logs:
    // let subscriber = tracing_subscriber::FmtSubscriber::builder()
    //     .with_max_level(tracing::Level::INFO)
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber)?;

    // Or use the library-provided logger setup that logs to file in `logs/` on your
    // project root, and also log to console
    // setup_logger()?;

    run_twitch_controller(|msg, chatter| async move {
        println!("{chatter}: {}", msg.text);
    })
    .await
}
```

---

## 📡 Handled Events

| Event Type | Description | Status |
|-------------|--------------|--------|
| `session_welcome` | Saves session ID and subscribes to chat | ✅ |
| `session_keepalive` | Keeps the connection alive | ✅ |
| `notification` | Handles `channel.chat.message` | ✅ |
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
USER_ID=your_client_id
BROADCASTER_ID=target_channel_id
```

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

`twitch-eventsub` never sets a global tracing subscriber (that would violate library crate rules).  
If you want logs with tracing, configure a global one yourself, since logging is already present
in my current code, or use the provided function `setup_logger()` exposed through global `prelude.rs`:

```rust
use tracing_subscriber::FmtSubscriber;

let subscriber = FmtSubscriber::builder()
    .with_max_level(tracing::Level::DEBUG)
    .finish();
tracing::subscriber::set_global_default(subscriber)?;
```

Or use the predefined logger that logs to file in `logs/` and to console with INFO level:
```rust
use twitch_eventsub::prelude::setup_logger;

setup_logger()?;
```

If you don’t, it’ll stay silent.

---

## 🤝 Contributing

Pull requests are **welcome**, but **I’ll only merge them if they align with the crate’s current goals and style.**  
This isn’t a general-purpose Twitch library — it’s a focused one.  
That said, if you’ve got a good addition or cleanup that fits well, I’ll happily review it.
Moreover you are free to fork this repository and maintain your own version.

---

## 📜 License

MIT © 2026  
Free to use, modify, and adapt with attribution.

---

## 💬 Author

Created by **pmrch** — just a self-learnt developer in high-school. Claude.ai also assisted in the WebSocket
setup with tokio-tungstenite. This project is part of a larger personal goal.
