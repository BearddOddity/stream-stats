// Minimal standalone credential store.
//
// The StreamerSuite-integrated version of this tool reuses StatusForge's
// full OAuth login (auth.rs — PKCE flow, token refresh, etc.) so users don't
// need a second login. Replicating that whole OAuth flow here for a
// standalone build is out of scope for this pass, so standalone users
// instead paste an already-obtained Client ID/Secret/Access Token directly
// (the same "Access Token (Optional)" manual path StatusForge itself offers
// as an alternative to its OAuth button) — stored locally as plain JSON in
// the app's data dir. No token refresh: if a pasted access token expires,
// paste a new one.
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Credentials {
    #[serde(default)]
    pub twitch_client_id: String,
    #[serde(default)]
    pub twitch_access_token: String,
    #[serde(default)]
    pub twitch_broadcaster_id: String,
    #[serde(default)]
    pub kick_access_token: String,
    #[serde(default)]
    pub kick_channel_slug: String,
}

fn credentials_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    use tauri::Manager;
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("credentials.json"))
}

#[tauri::command]
pub fn get_credentials(app: tauri::AppHandle) -> Result<Credentials, String> {
    let path = credentials_path(&app)?;
    if !path.exists() {
        return Ok(Credentials::default());
    }
    let raw = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_credentials(app: tauri::AppHandle, credentials: Credentials) -> Result<(), String> {
    let path = credentials_path(&app)?;
    let raw = serde_json::to_string_pretty(&credentials).map_err(|e| e.to_string())?;
    std::fs::write(&path, raw).map_err(|e| e.to_string())
}
