use harm_pm::manager::ProcessManager;
use slog::{o, Drain, Logger};
use tauri::{async_runtime::Mutex, App, Manager};

pub struct AppState {
    pub process_manager: ProcessManager,
    pub logger: Logger,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            init_state(app);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_state(app: &mut App) {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, o!());

    let process_manager = ProcessManager::new(
        String::from("~/.local/Steam/steamapps/common/Arma Reforger Server/ArmaReforgerServer"),
        logger.new(o!("name" => "process_manager")),
    );

    let state = AppState {
        process_manager,
        logger,
    };

    app.manage(Mutex::new(state));
}
