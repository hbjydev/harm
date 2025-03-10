use std::path::PathBuf;

use steamlocate::SteamDir;

const REFORGER_APP_ID: u32 = 4_000;

pub fn detect_reforger_path() -> Option<PathBuf> {
    let steam_dir = SteamDir::locate();
    if steam_dir.is_err() {
        return None;
    }
    let steam_dir = steam_dir.unwrap();

    let app_dir = steam_dir.find_app(REFORGER_APP_ID);
    if app_dir.is_err() {
        return None;
    }
    let app_dir = app_dir.unwrap();
    if app_dir.is_none() {
        return None;
    }

    let (app, lib) = app_dir.unwrap();

    Some(lib.resolve_app_dir(&app))
}
