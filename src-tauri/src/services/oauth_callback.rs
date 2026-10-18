use std::time::{Duration, SystemTime, UNIX_EPOCH};

use base64::Engine;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};
use sha2::{Digest, Sha256};
use tauri_plugin_keyring::KeyringExt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

use crate::services::llm_provider::KEYRING_SERVICE;

// A loopback client gets two seconds and 8 KiB to send one HTTP request line.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(2);
const MAX_REQUEST_LINE_BYTES: usize = 8 * 1024;

pub(super) async fn read_callback_url(stream: &mut TcpStream) -> Option<reqwest::Url> {
    let mut line = Vec::new();
    let mut reader = BufReader::new(stream).take((MAX_REQUEST_LINE_BYTES + 1) as u64);
    let count = tokio::time::timeout(REQUEST_TIMEOUT, reader.read_until(b'\n', &mut line))
        .await
        .ok()?
        .ok()?;
    if count > MAX_REQUEST_LINE_BYTES || !line.ends_with(b"\n") {
        return None;
    }
    let mut parts = std::str::from_utf8(&line).ok()?.split_whitespace();
    if parts.next()? != "GET" {
        return None;
    }
    let path = parts.next()?;
    if !path.starts_with('/') || path.starts_with("//") {
        return None;
    }
    if !matches!(parts.next()?, "HTTP/1.0" | "HTTP/1.1") || parts.next().is_some() {
        return None;
    }
    reqwest::Url::parse(&format!("http://localhost{path}")).ok()
}

pub(super) fn deserialize_interval_secs<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(number) => number
            .as_u64()
            .ok_or_else(|| de::Error::custom("interval must be a positive integer")),
        serde_json::Value::String(value) => value
            .trim()
            .parse::<u64>()
            .map_err(|err| de::Error::custom(format!("invalid interval: {err}"))),
        serde_json::Value::Null => Ok(0),
        _ => Err(de::Error::custom("interval must be a string or number")),
    }
}

pub(super) fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

pub(super) fn delete_keyring_password(
    app_handle: &tauri::AppHandle,
    user: &str,
    description: &str,
) -> Result<(), String> {
    match app_handle.keyring().delete_password(KEYRING_SERVICE, user) {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(_) => Err(format!("Failed to {description}")),
    }
}

pub(super) fn decode_jwt_claims<T: DeserializeOwned>(jwt: &str) -> Option<T> {
    let mut parts = jwt.split('.');
    let (_header, payload, _signature) = match (parts.next(), parts.next(), parts.next()) {
        (Some(h), Some(p), Some(s)) if !h.is_empty() && !p.is_empty() && !s.is_empty() => (h, p, s),
        _ => return None,
    };
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .ok()?;
    serde_json::from_slice(&bytes).ok()
}

pub(super) fn generate_code_verifier() -> String {
    OsRng
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

pub(super) fn base64_url_encode(bytes: &[u8]) -> String {
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

pub(super) fn generate_pkce_pair() -> (String, String) {
    let verifier = generate_code_verifier();
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = base64_url_encode(&hasher.finalize());
    (verifier, challenge)
}

pub(super) fn generate_state() -> String {
    let bytes = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(48)
        .collect::<Vec<u8>>();
    base64_url_encode(&bytes)
}

pub(super) fn html_escape(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

pub(super) fn callback_html(title: &str, message: &str, auto_close: bool) -> String {
    let script = if auto_close {
        "<script>setTimeout(() => window.close(), 1200)</script>"
    } else {
        ""
    };
    let title = html_escape(title);
    let message = html_escape(message);
    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>{title}</title><style>body{{margin:0;min-height:100vh;display:flex;align-items:center;justify-content:center;background:#111827;color:#f9fafb;font-family:-apple-system,BlinkMacSystemFont,'SF Pro Text','Helvetica Neue','Segoe UI',system-ui,sans-serif;font-weight:400;letter-spacing:-.008em;text-align:center}}h1{{margin:0;font-size:34px;line-height:1.2;font-weight:700;letter-spacing:-.02em}}p{{margin:0;font-size:21px;line-height:1.65;white-space:pre-wrap}}</style></head><body><div style=\"max-width:680px;padding:48px 32px;display:flex;flex-direction:column;align-items:center;gap:18px;\"><h1>{title}</h1><p>{message}</p></div>{script}</body></html>"
    )
}

pub(super) async fn respond_with_html(
    stream: &mut TcpStream,
    status_line: &str,
    html: &str,
) -> Result<(), String> {
    let body = html.as_bytes();
    let response = format!(
        "HTTP/1.1 {status_line}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream
        .write_all(response.as_bytes())
        .await
        .map_err(|err| format!("Failed to write OAuth callback response: {err}"))?;
    stream
        .write_all(body)
        .await
        .map_err(|err| format!("Failed to write OAuth callback response body: {err}"))?;
    stream
        .flush()
        .await
        .map_err(|err| format!("Failed to flush OAuth callback response: {err}"))
}
