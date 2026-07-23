mod credentials;
mod stream_stats;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            credentials::get_credentials,
            credentials::save_credentials,
            stream_stats::twitch_stream_stats,
            stream_stats::kick_channel_stats,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
