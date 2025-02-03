use config::AppConfig;
use tauri::{async_runtime::JoinHandle, AppHandle, Manager, State};

mod config;

#[tauri::command]
fn get_config() -> AppConfig {
    AppConfig::read()
}

#[tauri::command]
fn update_config(config: AppConfig) -> Result<(), String> {
    config
        .save()
        .map_err(|e| format!("Failed to update config: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            start_api,
            stop_api,
            get_config,
            update_config
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn start_api(app: AppHandle) -> Result<(), String> {
    let config = AppConfig::read();
    if config.reforger_path.is_none() {
        return Err(String::from("No reforger_path set!"));
    }

    let future =
        tauri::async_runtime::spawn(_start_api(config.api_port, config.reforger_path.unwrap()));

    app.manage(future);

    Ok(())
}

#[tauri::command]
async fn stop_api(state: State<'_, JoinHandle<Result<(), String>>>) -> Result<(), String> {
    state.abort();
    Ok(())
}

async fn _start_api(port: u16, reforger_path: String) -> Result<(), String> {
    let db_path = config::config_path();
    let db_url = format!("sqlite://{:?}?mode=rwc", db_path);
    harm_api::start(port, db_url, reforger_path).await
}
