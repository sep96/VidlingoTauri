use std::process::Command;
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
        .invoke_handler(tauri::generate_handler![install_ffmpeg, convert_video_to_mp4])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn convert_video_to_mp4(input_path: String) -> Result<String, String> {
    let output_path = input_path.replace(".mkv", ".mp4"); // A simple way to generate output path

    let status = Command::new("ffmpeg") // Assuming ffmpeg is in the system's PATH
        .arg("-i")
        .arg(&input_path)
        .arg("-c:v")
        .arg("libx264") // Video codec
        .arg("-c:a")
        .arg("aac")     // Audio codec
        .arg(&output_path)
        .status()
        .map_err(|e| e.to_string())?;

    if status.success() {
        Ok(output_path)
    } else {
        Err("FFmpeg conversion failed".to_string())
    }
}
