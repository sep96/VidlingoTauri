// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use ffmpeg_next as ffmpeg;

fn main() {
    vidlingo_lib::run()
}

#[tauri::command]
fn get_ffmpeg_version() -> String {
    ffmpeg::init().unwrap();
    ffmpeg::version::string().to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_ffmpeg_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}