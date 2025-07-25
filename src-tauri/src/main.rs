// src-tauri/src/main.rs
#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, Window};
use std::thread;
use std::time::{Duration, Instant};
use ffmpeg_next as ffmpeg;

// ساختاری برای ارسال اطلاعات اولیه ویدیو به فرانت‌اند
#[derive(Clone, serde::Serialize)]
struct VideoInfo {
  width: u32,
  height: u32,
}

// این کامند از فرانت‌اند فراخوانی می‌شود
#[tauri::command]
fn start_playback(window: Window, video_path: String) {
  // پردازش ویدیو در یک ترد جدید انجام می‌شود تا رابط کاربری قفل نشود
  thread::spawn(move || {
    if let Err(e) = process_video(window, video_path) {
        eprintln!("Error processing video: {}", e);
    }
  });
}

fn process_video(window: Window, path: String) -> Result<(), ffmpeg::Error> {
    ffmpeg::init()?;

    if let Ok(mut ictx) = ffmpeg::format::input(&path) {
        let input_stream = ictx
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or(ffmpeg::Error::StreamNotFound)?;
        let video_stream_index = input_stream.index();

        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
        let mut decoder = context_decoder.decoder().video()?;

        let width = decoder.width();
        let height = decoder.height();
        
        // ارسال ابعاد ویدیو به فرانت‌اند از طریق یک event
        window.emit("video_info", VideoInfo { width, height }).unwrap();

        // یک scaler برای تبدیل فرمت فریم‌ها به RGBA که برای canvas مناسب است
        let mut scaler = ffmpeg::software::scaling::Context::get(
            decoder.format(), width, height,
            ffmpeg::format::Pixel::RGBA, width, height,
            ffmpeg::software::scaling::Flags::BILINEAR,
        )?;

        // محاسبه زمان هر فریم برای پخش با سرعت مناسب
        let frame_rate = input_stream.avg_frame_rate();
        let frame_interval = if frame_rate.num() > 0 && frame_rate.den() > 0 {
            Duration::from_secs_f64(frame_rate.den() as f64 / frame_rate.num() as f64)
        } else {
            Duration::from_secs_f64(1.0 / 30.0) // مقدار پیش‌فرض در صورت نبود اطلاعات فریم‌ریت
        };

        let mut frame_time = Instant::now();
        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet)?;
                let mut decoded_frame = ffmpeg::util::frame::video::Video::empty();
                while decoder.receive_frame(&mut decoded_frame).is_ok() {
                    let mut rgba_frame = ffmpeg::util::frame::video::Video::empty();
                    scaler.run(&decoded_frame, &mut rgba_frame)?;
                    
                    // ارسال داده‌های فریم به فرانت‌اند از طریق event
                    window.emit("new_frame", rgba_frame.data(0).to_vec()).unwrap();
                    
                    // همگام‌سازی زمان پخش
                    let elapsed = frame_time.elapsed();
                    if elapsed < frame_interval {
                        thread::sleep(frame_interval - elapsed);
                    }
                    frame_time = Instant::now();
                }
            }
        }
    } else {
        return Err(ffmpeg::Error::InvalidData);
    }

    Ok(())
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![start_playback])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}