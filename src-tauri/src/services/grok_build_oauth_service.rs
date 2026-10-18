use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use tauri_plugin_keyring::KeyringExt;
#[cfg(test)]
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

use super::oauth_callback::{
    callback_html, decode_jwt_claims, delete_keyring_password, deserialize_interval_secs,
    generate_pkce_pair, generate_state, now_ms, respond_with_html,
};
use crate::services::llm_provider::KEYRING_SERVICE;
use crate::state::oauth_session::OAuthOperation;
use crate::state::user_profile::{LlmProviderConfig, XaiAuthMode};
use crate::state::AppState;
use crate::utils::paths;

// Bound headers and response-body reads while account switching waits for refresh.
const TOKEN_HTTP_TIMEOUT: Duration = Duration::from_secs(15);

const XAI_PROVIDER: &str = "xai";
const CLIENT_ID: &str = "b1a00492-073a-47ea-816f-4c329264a828";
const AUTHORIZE_URL: &str = "https://auth.x.ai/oauth2/authorize";
const TOKEN_URL: &str = "https://auth.x.ai/oauth2/token";
const DEVICE_CODE_URL: &str = "https://auth.x.ai/oauth2/device/code";
const SCOPE: &str = "openid profile email offline_access grok-cli:access api:access";
const CALLBACK_HOST: &str = "127.0.0.1";
const CALLBACK_PORT: u16 = 56121;
const CALLBACK_PATH: &str = "/callback";
const REDIRECT_URI: &str = "http://127.0.0.1:56121/callback";
const SESSION_KEYRING_USER: &str = "grok-build-oauth";
const SESSION_REFRESH_TOKEN_KEYRING_USER: &str = "grok-build-oauth-refresh-token";
const OAUTH_TIMEOUT_SECS: u64 = 5 * 60;
const REFRESH_SKEW_SECS: u64 = 60;
pub const GROK_BUILD_OAUTH_PREFIX: &str = "grok-build-oauth:";
#[allow(dead_code)]
pub const GROK_BUILD_INFERENCE_BASE_URL: &str = "https://cli-chat-proxy.grok.com";
#[allow(dead_code)]
pub const XAI_API_KEY_INFERENCE_BASE_URL: &str = "https://api.x.ai";
pub const GROK_BUILD_RESPONSES_URL: &str = "https://cli-chat-proxy.grok.com/v1/responses";
pub const GROK_BUILD_MODELS_URL: &str = "https://cli-chat-proxy.grok.com/v1/models";
pub const XAI_API_MODELS_URL: &str = "https://api.x.ai/v1/models";
pub const GROK_CLI_USER_AGENT: &str = "xai-grok-cli";
pub const GROK_CLI_CLIENT_IDENTIFIER: &str = "grok-shell";
pub const GROK_CLI_TOKEN_AUTH: &str = "xai-grok-cli";
pub const GROK_CLI_CLIENT_VERSION: &str = "0.2.114";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrokBuildOauthSession {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at_ms: Option<u64>,
    pub account_id: Option<String>,
    pub email: Option<String>,
    pub plan_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GrokBuildOauthStatus {
    pub logged_in: bool,
    pub email: Option<String>,
    pub plan_type: Option<String>,
    pub account_id: Option<String>,
    pub expires_at_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrokBuildOauthDeviceCodeChallenge {
    pub verification_url: String,
    pub user_code: String,
    pub device_code: String,
    pub interval_secs: u64,
}

fn device_challenge_binding(challenge: &GrokBuildOauthDeviceCodeChallenge) -> String {
    format!("{}\0{}", challenge.device_code, challenge.user_code)
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    #[serde(default)]
    id_token: Option<String>,
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default)]
    expires_in: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    #[serde(alias = "usercode")]
    user_code: String,
    #[serde(default, alias = "verification_url")]
    verification_uri: Option<String>,
    #[serde(default)]
    verification_uri_complete: Option<String>,
    #[serde(default, deserialize_with = "deserialize_interval_secs")]
    interval: u64,
}

#[derive(Debug, Deserialize, Default)]
struct OAuthErrorBody {
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
struct JwtClaims {
    #[serde(default)]
    exp: Option<u64>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    sub: Option<String>,
    #[serde(default)]
    preferred_username: Option<String>,
    #[serde(default)]
    plan_type: Option<String>,
    #[serde(default)]
    plan: Option<String>,
}

#[derive(Debug)]
struct OAuthCallback {
    code: String,
    stream: tokio::net::TcpStream,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PersistedGrokBuildOauthSession {
    #[serde(default)]
    pub logged_out: bool,
    pub expires_at_ms: Option<u64>,
    pub account_id: Option<String>,
    pub email: Option<String>,
    pub plan_type: Option<String>,
}

fn make_status(session: Option<&GrokBuildOauthSession>) -> GrokBuildOauthStatus {
    if let Some(session) = session {
        return GrokBuildOauthStatus {
            logged_in: true,
            email: session.email.clone(),
            plan_type: session.plan_type.clone(),
            account_id: session.account_id.clone(),
            expires_at_ms: session.expires_at_ms,
        };
    }

    GrokBuildOauthStatus::default()
}

fn session_meta_path() -> std::path::PathBuf {
    paths::get_data_dir().join("grok_build_oauth_session.json")
}

fn read_session_meta_at(
    path: &std::path::Path,
) -> Result<Option<PersistedGrokBuildOauthSession>, String> {
    match std::fs::read_to_string(path) {
        Ok(raw) => {
            let value: serde_json::Value = serde_json::from_str(&raw)
                .map_err(|_| "Failed to read Grok Build OAuth metadata".to_string())?;
            let object = value
                .as_object()
                .ok_or_else(|| "Failed to read Grok Build OAuth metadata".to_string())?;
            if !object.keys().any(|key| {
                matches!(
                    key.as_str(),
                    "logged_out" | "expires_at_ms" | "account_id" | "email" | "plan_type"
                )
            }) {
                return Err("Failed to read Grok Build OAuth metadata".to_string());
            }
            serde_json::from_value(value)
                .map(Some)
                .map_err(|_| "Failed to read Grok Build OAuth metadata".to_string())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("Failed to read Grok Build OAuth metadata".to_string()),
    }
}

fn read_session_meta() -> Result<Option<PersistedGrokBuildOauthSession>, String> {
    read_session_meta_at(&session_meta_path())
}

fn write_metadata_at(
    path: &std::path::Path,
    metadata: &PersistedGrokBuildOauthSession,
) -> Result<(), String> {
    let raw = serde_json::to_vec(metadata)
        .map_err(|_| "Failed to serialize Grok Build OAuth metadata".to_string())?;
    paths::atomic_write(path, &raw)
        .map_err(|_| "Failed to save Grok Build OAuth metadata".to_string())
}

fn write_session_meta(session: &GrokBuildOauthSession) -> Result<(), String> {
    let persisted = PersistedGrokBuildOauthSession {
        logged_out: false,
        expires_at_ms: session.expires_at_ms,
        account_id: session.account_id.clone(),
        email: session.email.clone(),
        plan_type: session.plan_type.clone(),
    };
    write_metadata_at(&session_meta_path(), &persisted)
}

fn write_logout_tombstone() -> Result<(), String> {
    let metadata = PersistedGrokBuildOauthSession {
        logged_out: true,
        ..Default::default()
    };
    write_metadata_at(&session_meta_path(), &metadata)
}

fn load_session_from_storage(app_handle: &tauri::AppHandle) -> Option<GrokBuildOauthSession> {
    let meta = match read_session_meta() {
        Ok(meta) => meta,
        Err(_) => {
            log::warn!("Ignoring unreadable Grok Build OAuth metadata");
            return None;
        }
    };
    if meta.as_ref().is_some_and(|meta| meta.logged_out) {
        return None;
    }
    let refresh_token = app_handle
        .keyring()
        .get_password(KEYRING_SERVICE, SESSION_REFRESH_TOKEN_KEYRING_USER)
        .ok()
        .flatten()
        .filter(|value| !value.trim().is_empty());

    if let (Some(meta), Some(refresh_token)) = (meta.as_ref(), refresh_token) {
        return Some(GrokBuildOauthSession {
            id_token: String::new(),
            access_token: String::new(),
            refresh_token,
            expires_at_ms: meta.expires_at_ms,
            account_id: meta.account_id.clone(),
            email: meta.email.clone(),
            plan_type: meta.plan_type.clone(),
        });
    }

    app_handle
        .keyring()
        .get_password(KEYRING_SERVICE, SESSION_KEYRING_USER)
        .ok()
        .flatten()
        .and_then(|raw| serde_json::from_str::<GrokBuildOauthSession>(&raw).ok())
        .filter(|session| {
            !session.access_token.trim().is_empty() && !session.refresh_token.trim().is_empty()
        })
}

fn save_session_to_storage(
    app_handle: &tauri::AppHandle,
    session: &GrokBuildOauthSession,
) -> Result<(), String> {
    write_logout_tombstone()?;
    app_handle
        .keyring()
        .set_password(
            KEYRING_SERVICE,
            SESSION_REFRESH_TOKEN_KEYRING_USER,
            &session.refresh_token,
        )
        .map_err(|_| "Failed to save Grok Build OAuth refresh token".to_string())?;
    delete_keyring_password(
        app_handle,
        SESSION_KEYRING_USER,
        "delete Grok Build OAuth legacy session",
    )?;
    // Commit valid metadata last; any earlier failure leaves the intent marker.
    write_session_meta(session)
}

fn clear_session_from_storage(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let mut errors = Vec::new();
    if let Err(error) = write_logout_tombstone() {
        errors.push(error);
    }
    if let Err(error) = delete_keyring_password(
        app_handle,
        SESSION_KEYRING_USER,
        "delete Grok Build OAuth legacy session",
    ) {
        errors.push(error);
    }
    if let Err(error) = delete_keyring_password(
        app_handle,
        SESSION_REFRESH_TOKEN_KEYRING_USER,
        "delete Grok Build OAuth refresh token",
    ) {
        errors.push(error);
    }
    if errors.is_empty() {
        match std::fs::remove_file(session_meta_path()) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => errors.push("Failed to delete Grok Build OAuth metadata".to_string()),
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

fn enrich_session_from_tokens(session: &mut GrokBuildOauthSession) {
    let claims = decode_jwt_claims::<JwtClaims>(&session.id_token)
        .or_else(|| decode_jwt_claims::<JwtClaims>(&session.access_token));
    if let Some(claims) = claims {
        session.email = claims
            .email
            .or(claims.preferred_username)
            .or_else(|| session.email.take());
        session.account_id = claims.sub.or_else(|| session.account_id.take());
        session.plan_type = claims
            .plan_type
            .or(claims.plan)
            .or_else(|| session.plan_type.take());
        if session.expires_at_ms.is_none() {
            session.expires_at_ms = claims.exp.map(|exp| exp.saturating_mul(1000));
        }
    }
}

pub fn encode_grok_build_oauth_access_token(access_token: &str) -> Option<String> {
    let access_token = access_token.trim();
    if access_token.is_empty() {
        return None;
    }
    Some(format!("{GROK_BUILD_OAUTH_PREFIX}{access_token}"))
}

pub fn decode_grok_build_oauth_access_token(input: &str) -> Option<String> {
    let payload = input.trim().strip_prefix(GROK_BUILD_OAUTH_PREFIX)?;
    let payload = payload.trim();
    if payload.is_empty() {
        return None;
    }
    Some(payload.to_string())
}

pub fn is_grok_build_oauth_origin_auth(input: &str) -> bool {
    decode_grok_build_oauth_access_token(input).is_some()
}

/// Grok Build CLI proxy has not advertised image input metadata.
/// Treat unknown support as text-only so polish/assistant do not attach
/// full-screen screenshots that stall the request for tens of seconds.
pub fn resolve_image_support_for_request(api_key: &str, observed: Option<bool>) -> Option<bool> {
    if observed.is_none() && is_grok_build_oauth_origin_auth(api_key) {
        return Some(false);
    }
    observed
}

pub fn effective_xai_auth_mode(stored_mode: Option<XaiAuthMode>, logged_in: bool) -> XaiAuthMode {
    stored_mode.unwrap_or(if logged_in {
        XaiAuthMode::Oauth
    } else {
        XaiAuthMode::ApiKey
    })
}

fn form_encode(pairs: &[(&str, &str)]) -> Result<String, String> {
    Ok(reqwest::Url::parse_with_params("http://localhost", pairs)
        .map_err(|err| format!("Failed to construct OAuth form: {err}"))?
        .query()
        .unwrap_or_default()
        .to_string())
}

async fn bind_callback_listener() -> Result<TcpListener, String> {
    TcpListener::bind((CALLBACK_HOST, CALLBACK_PORT))
        .await
        .map_err(|err| {
            format!(
                "Unable to bind Grok Build OAuth callback port {CALLBACK_PORT}. Please close the application using this port or switch to device code sign-in: {err}"
            )
        })
}

async fn wait_for_callback(
    listener: TcpListener,
    expected_state: String,
) -> Result<OAuthCallback, String> {
    wait_for_callback_with_timeout(
        listener,
        expected_state,
        Duration::from_secs(OAUTH_TIMEOUT_SECS),
    )
    .await
}

async fn wait_for_callback_with_timeout(
    listener: TcpListener,
    expected_state: String,
    timeout: Duration,
) -> Result<OAuthCallback, String> {
    tokio::time::timeout(timeout, async {
        loop {
            let (mut stream, _) = listener
                .accept()
                .await
                .map_err(|err| format!("Failed to accept Grok Build OAuth callback: {err}"))?;
            let Some(url) = super::oauth_callback::read_callback_url(&mut stream).await else {
                continue;
            };

            if url.path() != CALLBACK_PATH {
                let html = callback_html("Invalid Callback", "Unexpected callback path.", false);
                let _ = respond_with_html(&mut stream, "404 Not Found", &html).await;
                continue;
            }

            let query = url
                .query_pairs()
                .into_owned()
                .collect::<std::collections::HashMap<_, _>>();
            let state = query.get("state").map(String::as_str).unwrap_or_default();
            if state != expected_state {
                let html = callback_html(
                    "State Mismatch",
                    "Login state does not match the original request.",
                    false,
                );
                let _ = respond_with_html(&mut stream, "400 Bad Request", &html).await;
                continue;
            }

            if let Some(error_code) = query.get("error") {
                let description = query
                    .get("error_description")
                    .map(String::as_str)
                    .filter(|value| !value.trim().is_empty());
                let message = match description {
                    Some(description) => format!("Grok Build OAuth sign-in failed: {description}"),
                    None => format!("Grok Build OAuth sign-in failed: {error_code}"),
                };
                let html = callback_html("Authorization Failed", &message, false);
                let _ = respond_with_html(&mut stream, "200 OK", &html).await;
                return Err(message);
            }

            let Some(code) = query
                .get("code")
                .cloned()
                .filter(|value| !value.trim().is_empty())
            else {
                let html = callback_html(
                    "Missing Code",
                    "Authorization code was not returned.",
                    false,
                );
                let _ = respond_with_html(&mut stream, "400 Bad Request", &html).await;
                continue;
            };

            return Ok(OAuthCallback { code, stream });
        }
    })
    .await
    .map_err(|_| "Grok Build OAuth callback timed out. Please try again.".to_string())?
}

fn build_authorize_url(code_challenge: &str, state: &str) -> Result<String, String> {
    let url = reqwest::Url::parse_with_params(
        AUTHORIZE_URL,
        &[
            ("response_type", "code"),
            ("client_id", CLIENT_ID),
            ("redirect_uri", REDIRECT_URI),
            ("scope", SCOPE),
            ("code_challenge", code_challenge),
            ("code_challenge_method", "S256"),
            ("state", state),
        ],
    )
    .map_err(|err| format!("Failed to construct Grok Build OAuth URL: {err}"))?;
    Ok(url.to_string())
}

fn oauth_http_error(prefix: &str, status: reqwest::StatusCode, body: &str) -> String {
    if let Ok(payload) = serde_json::from_str::<OAuthErrorBody>(body) {
        if let Some(description) = payload
            .error_description
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            return format!("{prefix}: {description}");
        }
        if let Some(error) = payload
            .error
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            return format!("{prefix}: {error}");
        }
    }
    format!("{prefix} {status}: {body}")
}

fn should_invalidate_refresh(status: reqwest::StatusCode, body: &str) -> bool {
    if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
        return false;
    }
    if status.is_client_error() {
        return true;
    }
    body.to_ascii_lowercase().contains("invalid_grant")
}

async fn exchange_code_for_tokens(
    client: &reqwest::Client,
    code: &str,
    code_verifier: &str,
) -> Result<TokenResponse, String> {
    let response = client
        .post(TOKEN_URL)
        .timeout(TOKEN_HTTP_TIMEOUT)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_encode(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", REDIRECT_URI),
            ("client_id", CLIENT_ID),
            ("code_verifier", code_verifier),
        ])?)
        .send()
        .await
        .map_err(|err| format!("Failed to exchange OAuth authorization code for token: {err}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(oauth_http_error(
            "Failed to exchange OAuth authorization code for token",
            status,
            &body,
        ));
    }

    response
        .json::<TokenResponse>()
        .await
        .map_err(|err| format!("Failed to parse OAuth token response: {err}"))
}

async fn request_device_code(
    client: &reqwest::Client,
) -> Result<GrokBuildOauthDeviceCodeChallenge, String> {
    let response = client
        .post(DEVICE_CODE_URL)
        .timeout(TOKEN_HTTP_TIMEOUT)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_encode(&[("client_id", CLIENT_ID), ("scope", SCOPE)])?)
        .send()
        .await
        .map_err(|err| format!("Failed to request Grok Build device code: {err}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(oauth_http_error(
            "Failed to request Grok Build device code",
            status,
            &body,
        ));
    }

    let payload = response
        .json::<DeviceCodeResponse>()
        .await
        .map_err(|err| format!("Failed to parse Grok Build device code response: {err}"))?;
    let verification_url = payload
        .verification_uri
        .filter(|value| !value.trim().is_empty())
        .or(payload.verification_uri_complete.clone())
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "Grok Build device code response is missing verification URL".to_string())?;

    Ok(GrokBuildOauthDeviceCodeChallenge {
        verification_url,
        user_code: payload.user_code,
        device_code: payload.device_code,
        interval_secs: payload.interval.max(1),
    })
}

async fn poll_device_code_token(
    client: &reqwest::Client,
    challenge: &GrokBuildOauthDeviceCodeChallenge,
) -> Result<TokenResponse, String> {
    let max_wait = Duration::from_secs(15 * 60);
    let mut interval = Duration::from_secs(challenge.interval_secs.clamp(1, 30));
    let start = Instant::now();

    loop {
        let response = client
            .post(TOKEN_URL)
            .timeout(TOKEN_HTTP_TIMEOUT)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_encode(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
                ("device_code", challenge.device_code.as_str()),
                ("client_id", CLIENT_ID),
            ])?)
            .send()
            .await
            .map_err(|err| format!("Failed to poll Grok Build device code authorization: {err}"))?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        if status.is_success() {
            return serde_json::from_str::<TokenResponse>(&body).map_err(|err| {
                format!("Failed to parse Grok Build device code token response: {err}")
            });
        }

        let error = serde_json::from_str::<OAuthErrorBody>(&body)
            .ok()
            .and_then(|payload| payload.error)
            .unwrap_or_default();
        match error.as_str() {
            "authorization_pending" => {}
            "slow_down" => {
                interval = interval.saturating_add(Duration::from_secs(5));
            }
            _ => {
                return Err(oauth_http_error(
                    "Grok Build device code authorization failed",
                    status,
                    &body,
                ));
            }
        }

        if start.elapsed() >= max_wait {
            return Err(
                "Grok Build device code sign-in timed out. Please start sign-in again.".to_string(),
            );
        }
        let remaining = max_wait.saturating_sub(start.elapsed());
        tokio::time::sleep(interval.min(remaining)).await;
    }
}

struct RefreshFailure {
    message: String,
    invalidate_session: bool,
}

async fn refresh_tokens(
    client: &reqwest::Client,
    refresh_token: &str,
) -> Result<TokenResponse, RefreshFailure> {
    refresh_tokens_at(client, TOKEN_URL, refresh_token, TOKEN_HTTP_TIMEOUT).await
}

async fn refresh_tokens_at(
    client: &reqwest::Client,
    endpoint: &str,
    refresh_token: &str,
    timeout: Duration,
) -> Result<TokenResponse, RefreshFailure> {
    let form = form_encode(&[
        ("grant_type", "refresh_token"),
        ("refresh_token", refresh_token),
        ("client_id", CLIENT_ID),
    ])
    .map_err(|message| RefreshFailure {
        message,
        invalidate_session: false,
    })?;
    let response = client
        .post(endpoint)
        .timeout(timeout)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form)
        .send()
        .await
        .map_err(|err| RefreshFailure {
            message: format!("Failed to refresh Grok Build OAuth token: {err}"),
            invalidate_session: false,
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(RefreshFailure {
            message: oauth_http_error("Failed to refresh Grok Build OAuth token", status, &body),
            invalidate_session: should_invalidate_refresh(status, &body),
        });
    }

    response
        .json::<TokenResponse>()
        .await
        .map_err(|err| RefreshFailure {
            message: format!("Failed to parse refresh token response: {err}"),
            invalidate_session: false,
        })
}

fn session_needs_refresh(session: &GrokBuildOauthSession) -> bool {
    match session.expires_at_ms {
        Some(expires_at_ms) => expires_at_ms <= now_ms().saturating_add(REFRESH_SKEW_SECS * 1000),
        None => false,
    }
}

fn session_has_runtime_auth_material(session: &GrokBuildOauthSession) -> bool {
    !session.access_token.trim().is_empty()
}

fn session_from_token_response(
    token_response: TokenResponse,
    previous: Option<&GrokBuildOauthSession>,
) -> Result<GrokBuildOauthSession, String> {
    let access_token = token_response.access_token.trim().to_string();
    if access_token.is_empty() {
        return Err("OAuth response is missing access_token. Cannot continue.".to_string());
    }
    let refresh_token = token_response
        .refresh_token
        .filter(|value| !value.trim().is_empty())
        .or_else(|| previous.map(|session| session.refresh_token.clone()))
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| "OAuth response is missing refresh_token. Cannot continue.".to_string())?;

    let mut session = GrokBuildOauthSession {
        id_token: token_response
            .id_token
            .filter(|value| !value.trim().is_empty())
            .or_else(|| previous.map(|session| session.id_token.clone()))
            .unwrap_or_default(),
        access_token,
        refresh_token,
        expires_at_ms: token_response
            .expires_in
            .map(|expires_in| now_ms().saturating_add(expires_in * 1000))
            .or_else(|| previous.and_then(|session| session.expires_at_ms)),
        account_id: previous.and_then(|session| session.account_id.clone()),
        email: previous.and_then(|session| session.email.clone()),
        plan_type: previous.and_then(|session| session.plan_type.clone()),
    };
    enrich_session_from_tokens(&mut session);
    Ok(session)
}

fn persist_login_session(
    app_handle: &tauri::AppHandle,
    state: &AppState,
    operation: OAuthOperation,
    session: GrokBuildOauthSession,
) -> Result<GrokBuildOauthStatus, String> {
    let status = make_status(Some(&session));
    let committed = state
        .grok_build_oauth_state()
        .commit(operation, session, |session| {
            save_session_to_storage(app_handle, session)
        })?;
    if !committed {
        return Err(
            "Grok Build OAuth sign-in superseded by a newer operation. Please try again."
                .to_string(),
        );
    }
    Ok(status)
}

pub fn should_prewarm_runtime_session(
    provider: &str,
    config: &LlmProviderConfig,
    session: Option<&GrokBuildOauthSession>,
) -> bool {
    if provider != XAI_PROVIDER {
        return false;
    }
    let Some(session) = session else {
        return false;
    };
    if effective_xai_auth_mode(config.xai_auth_mode, true) != XaiAuthMode::Oauth {
        return false;
    }
    session_needs_refresh(session) || !session_has_runtime_auth_material(session)
}

async fn refresh_session_if_needed(
    app_handle: &tauri::AppHandle,
    state: &AppState,
) -> Result<Option<GrokBuildOauthSession>, String> {
    refresh_session(app_handle, state, None).await
}

async fn refresh_session(
    app_handle: &tauri::AppHandle,
    state: &AppState,
    rejected: Option<&str>,
) -> Result<Option<GrokBuildOauthSession>, String> {
    let _refresh_guard = state.grok_build_oauth_state().lock_refresh().await;
    let Some((operation, session)) = state.grok_build_oauth_state().snapshot_for_refresh() else {
        return Ok(None);
    };
    if let Some(old) = rejected {
        if session_has_runtime_auth_material(&session) && session.access_token != old {
            let old_identity = decode_jwt_claims::<JwtClaims>(old).and_then(|c| c.sub);
            if old_identity.is_none() || old_identity != session.account_id {
                return Err("Grok account switched. Please retry the current operation.".into());
            }
            return Ok(Some(session));
        }
    }
    if rejected.is_none()
        && !session_needs_refresh(&session)
        && session_has_runtime_auth_material(&session)
    {
        return Ok(Some(session));
    }
    if session.refresh_token.trim().is_empty() {
        return Err(
            "Grok Build OAuth session is missing a refresh token. Please sign in again."
                .to_string(),
        );
    }

    let token_response = match refresh_tokens(&state.http_client, &session.refresh_token).await {
        Ok(token_response) => token_response,
        Err(err) => {
            if err.invalidate_session {
                match state
                    .grok_build_oauth_state()
                    .invalidate(Some(operation), || clear_session_from_storage(app_handle))
                {
                    Ok(false) => {
                        return Err(err.message);
                    }
                    Ok(true) => {}
                    Err(clear_error) => {
                        return Err(format!("{}; {clear_error}", err.message));
                    }
                }
                return Err("Grok Build sign-in expired. Please sign in again.".to_string());
            }
            return Err(err.message);
        }
    };

    let refreshed = session_from_token_response(token_response, Some(&session))?;
    let committed =
        state
            .grok_build_oauth_state()
            .commit(operation, refreshed.clone(), |session| {
                save_session_to_storage(app_handle, session)
            })?;
    if !committed {
        return Err(
            "Grok Build OAuth session superseded by a newer operation. Please try again."
                .to_string(),
        );
    }
    Ok(Some(refreshed))
}

pub async fn recover_rejected_auth(
    app: &tauri::AppHandle,
    state: &AppState,
    rejected: &str,
) -> Result<String, String> {
    let token =
        decode_grok_build_oauth_access_token(rejected).ok_or("Invalid Grok sign-in credentials")?;
    let session = refresh_session(app, state, Some(&token))
        .await?
        .ok_or("Grok sign-in is updating. Please try again.")?;
    encode_grok_build_oauth_access_token(&session.access_token)
        .ok_or("Grok sign-in is missing a token. Please sign in again.".into())
}

pub fn sync_runtime_session(
    app_handle: &tauri::AppHandle,
    state: &AppState,
) -> Option<GrokBuildOauthSession> {
    let login = state.grok_build_oauth_state().begin_login();
    let operation = login.token();
    let loaded = load_session_from_storage(app_handle);
    let result = match loaded {
        Some(session) => state
            .grok_build_oauth_state()
            .commit(operation, session, |_| Ok(())),
        None => Ok(false),
    };
    drop(login);
    if let Err(error) = result {
        log::warn!("Failed to restore Grok Build OAuth session: {error}");
    }
    state.read_grok_build_oauth_session()
}

pub fn status(state: &AppState) -> GrokBuildOauthStatus {
    make_status(state.read_grok_build_oauth_session().as_ref())
}

pub async fn prewarm_runtime_session(
    app_handle: &tauri::AppHandle,
    state: &AppState,
) -> Result<(), String> {
    let provider = state.active_llm_provider();
    let config = state.llm_provider_config();
    let session = state.read_grok_build_oauth_session();
    if !should_prewarm_runtime_session(&provider, &config, session.as_ref()) {
        return Ok(());
    }
    refresh_session_if_needed(app_handle, state).await?;
    Ok(())
}

pub async fn login(
    app_handle: &tauri::AppHandle,
    state: &AppState,
) -> Result<GrokBuildOauthStatus, String> {
    let login = state
        .grok_build_oauth_state()
        .begin_login_after_refresh()
        .await;
    let operation = login.token();
    let listener = bind_callback_listener().await?;
    let (code_verifier, code_challenge) = generate_pkce_pair();
    let state_token = generate_state();
    let auth_url = build_authorize_url(&code_challenge, &state_token)?;

    webbrowser::open(&auth_url).map_err(|err| format!("Failed to open browser: {err}"))?;

    let OAuthCallback { code, mut stream } = wait_for_callback(listener, state_token).await?;
    let token_response =
        match exchange_code_for_tokens(&state.http_client, &code, &code_verifier).await {
            Ok(tokens) => tokens,
            Err(err) => {
                let html = callback_html("Authorization Failed", &err, false);
                let _ = respond_with_html(&mut stream, "200 OK", &html).await;
                return Err(err);
            }
        };

    let session = match session_from_token_response(token_response, None) {
        Ok(session) => session,
        Err(err) => {
            let html = callback_html("Authorization Failed", &err, false);
            let _ = respond_with_html(&mut stream, "200 OK", &html).await;
            return Err(err);
        }
    };

    let status = match persist_login_session(app_handle, state, operation, session) {
        Ok(status) => status,
        Err(err) => {
            let html = callback_html("Authorization Failed", &err, false);
            let _ = respond_with_html(&mut stream, "200 OK", &html).await;
            return Err(err);
        }
    };
    drop(login);
    let html = callback_html(
        "Authorization Successful",
        "You can close this page and return to SpeakFlow.",
        true,
    );
    let _ = respond_with_html(&mut stream, "200 OK", &html).await;
    Ok(status)
}

pub async fn start_device_code_login(
    state: &AppState,
) -> Result<GrokBuildOauthDeviceCodeChallenge, String> {
    let login = state
        .grok_build_oauth_state()
        .begin_login_after_refresh()
        .await;
    let operation = login.token();
    let challenge = request_device_code(&state.http_client).await?;
    if challenge.device_code.trim().is_empty() || challenge.user_code.trim().is_empty() {
        return Err("Invalid Grok Build device code response. Please try again.".to_string());
    }
    let published = state
        .grok_build_oauth_state()
        .publish_challenge(operation, device_challenge_binding(&challenge))?;
    if !published {
        return Err(
            "Grok Build device code sign-in superseded by a newer operation. Please try again."
                .to_string(),
        );
    }
    drop(login);
    if let Err(err) = webbrowser::open(&challenge.verification_url) {
        log::warn!(
            "Failed to open Grok Build device code verification page; UI will display URL: {}",
            err
        );
    }
    Ok(challenge)
}

pub async fn complete_device_code_login(
    app_handle: &tauri::AppHandle,
    state: &AppState,
    challenge: GrokBuildOauthDeviceCodeChallenge,
) -> Result<GrokBuildOauthStatus, String> {
    if challenge.device_code.trim().is_empty() || challenge.user_code.trim().is_empty() {
        return Err("Grok Build device code expired. Please start sign-in again.".to_string());
    }
    let binding = device_challenge_binding(&challenge);
    let login = state
        .grok_build_oauth_state()
        .claim_challenge(&binding)
        .ok_or_else(|| "Grok Build device code expired. Please start sign-in again.".to_string())?;
    let operation = login.token();
    let token_response = poll_device_code_token(&state.http_client, &challenge).await?;
    let session = session_from_token_response(token_response, None)?;
    let result = persist_login_session(app_handle, state, operation, session);
    drop(login);
    result
}

pub fn logout(app_handle: &tauri::AppHandle, state: &AppState) -> Result<(), String> {
    state
        .grok_build_oauth_state()
        .invalidate(None, || clear_session_from_storage(app_handle))
        .map(|_| ())
}

pub fn grok_cli_request_headers(access_token: &str) -> Result<reqwest::header::HeaderMap, String> {
    use reqwest::header::{
        HeaderMap, HeaderName, HeaderValue, AUTHORIZATION, CONTENT_TYPE, USER_AGENT,
    };

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {access_token}"))
            .map_err(|err| format!("Grok Build token contains invalid characters: {err}"))?,
    );
    headers.insert(USER_AGENT, HeaderValue::from_static(GROK_CLI_USER_AGENT));
    headers.insert(
        HeaderName::from_static("x-grok-client-identifier"),
        HeaderValue::from_static(GROK_CLI_CLIENT_IDENTIFIER),
    );
    headers.insert(
        "X-XAI-Token-Auth",
        HeaderValue::from_static(GROK_CLI_TOKEN_AUTH),
    );
    headers.insert(
        HeaderName::from_static("x-grok-client-version"),
        HeaderValue::from_static(GROK_CLI_CLIENT_VERSION),
    );
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    Ok(headers)
}

pub async fn resolve_oauth_origin_api_key(
    app_handle: &tauri::AppHandle,
    state: &AppState,
) -> Result<String, String> {
    if state.read_grok_build_oauth_session().is_none() {
        return Ok(String::new());
    }
    let Some(session) = refresh_session_if_needed(app_handle, state).await? else {
        return Ok(String::new());
    };
    Ok(encode_grok_build_oauth_access_token(&session.access_token).unwrap_or_default())
}

#[cfg(test)]
mod storage_tests {
    use super::*;

    #[tokio::test]
    async fn refresh_deadline_releases_login_waiter_for_stalled_headers_and_body() {
        use tokio::io::AsyncReadExt;
        for send_headers in [false, true] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let endpoint = format!("http://{}/token", listener.local_addr().unwrap());
            let server = tokio::spawn(async move {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut buffer = [0u8; 2048];
                assert!(socket.read(&mut buffer).await.unwrap() > 0);
                if send_headers {
                    socket.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 256\r\n\r\n{").await.unwrap();
                }
                std::future::pending::<()>().await;
            });
            let coordinator = crate::state::oauth_session::OAuthSessionState::default();
            coordinator.restore(Some("original-refresh-token".to_string()));
            let guard = coordinator.lock_refresh().await;
            let client = reqwest::Client::new();
            let outcome = tokio::time::timeout(Duration::from_secs(1), async {
                tokio::join!(
                    async {
                        let result = refresh_tokens_at(
                            &client,
                            &endpoint,
                            "test-refresh",
                            Duration::from_millis(50),
                        )
                        .await;
                        drop(guard);
                        result
                    },
                    coordinator.begin_login_after_refresh()
                )
            })
            .await;
            server.abort();
            let (result, login) = outcome.expect("refresh deadline must release the login waiter");
            let error = result.unwrap_err();
            assert!(
                !error.invalidate_session,
                "network timeout must preserve credentials"
            );
            drop(login);
            assert_eq!(
                coordinator.read().as_deref(),
                Some("original-refresh-token")
            );
        }
    }

    #[tokio::test]
    async fn callback_total_deadline_includes_a_stalled_request_line() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();

        let mut stalled = tokio::net::TcpStream::connect(address).await.unwrap();
        stalled.write_all(b"GET /unfinished").await.unwrap();
        let result = tokio::time::timeout(
            Duration::from_millis(600),
            wait_for_callback_with_timeout(listener, "expected".into(), Duration::from_millis(200)),
        )
        .await;
        let Err(error) = result.expect("whole callback deadline must include accepted connections")
        else {
            panic!("partial line must not authorize login");
        };
        assert!(error.contains("timed out"), "{error}");
    }

    #[tokio::test]
    async fn callback_ignores_unrelated_and_incomplete_requests() {
        use tokio::net::TcpStream;
        for request in [
            "GET /favicon.ico HTTP/1.1\r\n".to_string(),
            "GET /callback?state=wrong&code=bad HTTP/1.1\r\n".to_string(),
            "G".repeat(9_000),
            "GET /unfinished".to_string(),
        ] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let callback = tokio::spawn(wait_for_callback(listener, "expected".into()));
            let mut unrelated = TcpStream::connect(address).await.unwrap();
            unrelated.write_all(request.as_bytes()).await.unwrap();
            let mut valid = TcpStream::connect(address).await.unwrap();
            let _ = valid
                .write_all(b"GET /callback?state=expected&code=good HTTP/1.1\r\n\r\n")
                .await;
            let result = tokio::time::timeout(Duration::from_secs(4), callback)
                .await
                .expect("unrelated connection must not block a valid callback")
                .unwrap();
            assert_eq!(result.map(|callback| callback.code).unwrap(), "good");
        }
    }

    #[test]
    fn device_challenge_binding_uses_server_device_id_and_user_code() {
        let challenge = GrokBuildOauthDeviceCodeChallenge {
            verification_url: "https://example.invalid".to_string(),
            user_code: "USER-CODE".to_string(),
            device_code: "device-code".to_string(),
            interval_secs: 5,
        };

        assert_eq!(
            device_challenge_binding(&challenge),
            "device-code\0USER-CODE"
        );
    }

    #[test]
    fn oauth_metadata_rejects_malformed_json_and_non_file_atomic_targets() {
        let root = std::env::temp_dir().join(format!(
            "speak-flow-grok-oauth-{}-{}",
            std::process::id(),
            now_ms()
        ));
        std::fs::create_dir_all(&root).expect("temporary metadata directory");
        let metadata_path = root.join("session.json");
        assert!(read_session_meta_at(&metadata_path)
            .expect("missing metadata is distinguishable")
            .is_none());
        crate::utils::paths::atomic_write(&metadata_path, b"{malformed")
            .expect("malformed metadata");

        assert!(read_session_meta_at(&metadata_path).is_err());
        crate::utils::paths::atomic_write(&metadata_path, b"{}").expect("empty metadata");
        assert!(read_session_meta_at(&metadata_path).is_err());

        let target_directory = root.join("target-directory");
        std::fs::create_dir_all(&target_directory).expect("atomic target directory");
        let metadata = PersistedGrokBuildOauthSession::default();
        assert!(write_metadata_at(&target_directory, &metadata).is_err());

        std::fs::remove_dir_all(root).expect("temporary metadata cleanup");
    }
}
