use std::process::Command;
use std::path::Path;
use std::fs;

#[tauri::command]
async fn install_ffmpeg() -> Result<String, String> {
    // Use a more appropriate directory for Windows
    let target_dir = "C:\\ffmpeg"; 
    let ffmpeg_url = "https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip";

    // Create target directory if it doesn't exist
    if !Path::new(target_dir).exists() {
        fs::create_dir_all(target_dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // 1. Download the file
    let response = reqwest::get(ffmpeg_url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;

    // 2. Unzip the file
    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

    // Extract using Path::new() to convert string to Path
    archive.extract(Path::new(target_dir)).map_err(|e| e.to_string())?;

    Ok(format!("FFmpeg installed successfully in {}", target_dir))
}

#[tauri::command]
async fn convert_video_to_mp4(input_path: String) -> Result<String, String> {
    let output_path = input_path.replace(".mkv", ".mp4");

    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(&input_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .arg(&output_path)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(output_path)
    } else {
        Err("FFmpeg conversion failed".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![install_ffmpeg, convert_video_to_mp4])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}