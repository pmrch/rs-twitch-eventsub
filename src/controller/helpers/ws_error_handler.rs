use super::TungError;

pub fn handle_ws_error(err: TungError) -> ErrorAction {
    match err {
        TungError::ConnectionClosed => {
            tracing::error!("WebSocket connection has been disrupted!");
            ErrorAction::Reconnect
        }
        TungError::Protocol(p) => {
            tracing::error!("{p}");
            ErrorAction::Reconnect
        }
        TungError::Io(i) => {
            tracing::error!("{i}");
            ErrorAction::Reconnect
        }
        TungError::AttackAttempt => {
            tracing::warn!("Someone tried to attack the WebSocket xD");
            ErrorAction::Skip
        }
        TungError::Utf8(u) => {
            tracing::warn!("{u}");
            ErrorAction::Skip
        }
        TungError::WriteBufferFull(wbf) => {
            tracing::warn!("{wbf}");
            ErrorAction::Skip
        }
        TungError::Capacity(c) => {
            tracing::error!("Message size exceeding maximum has been sent: {c}");
            ErrorAction::Skip
        }
        TungError::AlreadyClosed => {
            let err: anyhow::Error = anyhow::anyhow!("WebSocket error, tried using a dead socket!");
            tracing::error!("{err}");
            ErrorAction::Fatal(err.into())
        }
        TungError::Url(u) => {
            tracing::error!("{u}");
            ErrorAction::Fatal(anyhow::anyhow!(u).into())
        }
        TungError::Tls(t) => {
            tracing::error!("TLS error, fatal: {t}");
            ErrorAction::Fatal(anyhow::anyhow!(t).into())
        }
        TungError::Http(h) => {
            let err: anyhow::Error = anyhow::anyhow!("Handshake failed with status {}", h.status());
            tracing::error!("{err}");
            ErrorAction::Fatal(err.into())
        }
        TungError::HttpFormat(hf) => {
            tracing::error!("HTTP format error for WebSocket, fatal error: {hf}");
            ErrorAction::Fatal(anyhow::anyhow!(hf).into())
        }
    }
}

pub enum ErrorAction {
    Reconnect,
    Skip,
    Fatal(crate::prelude::Error),
}
