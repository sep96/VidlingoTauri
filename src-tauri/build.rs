// src-tauri/build.rs
fn main() {
    // این اسکریپت خالی به Rust کمک می‌کند تا از متغیرهای محیطی
    // برای پیدا کردن کتابخانه‌های FFmpeg استفاده کند.
    // پکیج ffmpeg-next به صورت خودکار از FFMPEG_DIR استفاده می‌کند.
    println!("cargo:rerun-if-changed=build.rs");
}