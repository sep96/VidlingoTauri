// In src-tauri/src/main.rs

#[tauri::command]
async fn install_ffmpeg() -> Result<String, String> {
    let target_dir = "path/to/install/ffmpeg"; // مسیر نصب
    let ffmpeg_url = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip"; // URL دانلود

    // 1. Download the file
    let response = reqwest::get(ffmpeg_url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;

    // 2. Unzip the file
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

    archive.extract(target_dir).map_err(|e| e.to_string())?;

    Ok(format!("FFmpeg installed successfully in {}", target_dir))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install_ffmpeg])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}