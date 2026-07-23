// Stream Stats — live viewer count, follower total, and subscriber total.
// See src/credentials.rs for how tokens are sourced in this standalone
// build (manually-pasted, not full OAuth).
use crate::credentials::get_credentials;
use serde_json::Value;

#[tauri::command]
pub(crate) async fn twitch_stream_stats(app: tauri::AppHandle) -> Result<Value, String> {
    let creds = get_credentials(app)?;
    if creds.twitch_access_token.is_empty() || creds.twitch_broadcaster_id.is_empty() {
        return Err("Twitch not connected — paste a Client ID, Access Token, and Broadcaster ID in Settings first".into());
    }
    let access_token = &creds.twitch_access_token;
    let client_id = &creds.twitch_client_id;
    let user_id = &creds.twitch_broadcaster_id;

    async fn get(client: &reqwest::Client, url: &str, params: &[(&str, &str)], access_token: &str, client_id: &str) -> Result<Value, String> {
        let resp = client
            .get(url)
            .query(params)
            .bearer_auth(access_token)
            .header("Client-Id", client_id)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        resp.json().await.map_err(|e| e.to_string())
    }

    let client = reqwest::Client::new();
    let stream_params = [("user_id", user_id.as_str())];
    let follower_params = [("broadcaster_id", user_id.as_str())];
    let sub_params = [("broadcaster_id", user_id.as_str())];
    let (stream_resp, followers_resp, subs_resp) = tokio::join!(
        get(&client, "https://api.twitch.tv/helix/streams", &stream_params, access_token, client_id),
        get(&client, "https://api.twitch.tv/helix/channels/followers", &follower_params, access_token, client_id),
        get(&client, "https://api.twitch.tv/helix/subscriptions", &sub_params, access_token, client_id),
    );

    let stream = stream_resp.ok().and_then(|v| v.pointer("/data/0").cloned());
    let follower_total = followers_resp.ok().and_then(|v| v.get("total").and_then(|t| t.as_i64()));
    // A stream not run by a Partner/Affiliate 400s on /subscriptions — that's
    // not a real error for stats purposes, just "no sub program", so it's
    // folded into `null` rather than failing the whole combined response.
    let subscriber_total = subs_resp.ok().and_then(|v| v.get("total").and_then(|t| t.as_i64()));

    Ok(serde_json::json!({
        "is_live": stream.is_some(),
        "viewer_count": stream.as_ref().and_then(|s| s.get("viewer_count")),
        "title": stream.as_ref().and_then(|s| s.get("title")),
        "game_name": stream.as_ref().and_then(|s| s.get("game_name")),
        "started_at": stream.as_ref().and_then(|s| s.get("started_at")),
        "follower_total": follower_total,
        "subscriber_total": subscriber_total,
    }))
}

#[tauri::command]
pub(crate) async fn kick_channel_stats(app: tauri::AppHandle, slug: String) -> Result<Value, String> {
    let creds = get_credentials(app)?;
    if creds.kick_access_token.is_empty() {
        return Err("Kick not connected — paste an Access Token in Settings first".into());
    }

    let client = reqwest::Client::new();
    let resp = client
        .get("https://api.kick.com/public/v1/channels")
        .query(&[("slug", &slug)])
        .bearer_auth(&creds.kick_access_token)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("Kick returned {}", resp.status()));
    }
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;
    let channel = body
        .pointer("/data/0")
        .ok_or_else(|| format!("couldn't find Kick channel \"{slug}\""))?;

    Ok(serde_json::json!({
        "is_live": channel.pointer("/stream/is_live").and_then(|v| v.as_bool()).unwrap_or(false),
        "viewer_count": channel.pointer("/stream/viewer_count"),
        "title": channel.get("stream_title"),
        "category_name": channel.pointer("/category/name"),
        "started_at": channel.pointer("/stream/start_time"),
    }))
}
