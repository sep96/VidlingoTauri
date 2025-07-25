// main.rs

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Manager, Window};
use std::thread;
use std::time::Duration;
use ffmpeg_next as ffmpeg;

// این کامند از فرانت‌اند فراخوانی می‌شود
#[tauri::command]
fn start_playback(window: Window, video_path: String) {
  // اجرای پردازش ویدیو در یک ترد جداگانه تا رابط کاربری قفل نشود
  thread::spawn(move || {
    // مقداردهی اولیه ffmpeg
    ffmpeg::init().unwrap();

    // باز کردن فایل ویدیو
    match ffmpeg::format::input(&video_path) {
      Ok(mut ictx) => {
        // پیدا کردن استریم ویدیو
        let input_stream = ictx
          .streams()
          .best(ffmpeg::media::Type::Video)
          .expect("Could not find video stream.");
        let video_stream_index = input_stream.index();

        // ساخت دیکودر
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters()).unwrap();
        let mut decoder = context_decoder.decoder().video().unwrap();

        // گرفتن نرخ فریم (FPS) برای همگام‌سازی پخش
        let frame_rate = input_stream.avg_frame_rate();
        let frame_interval = Duration::from_secs_f64(1.0 / frame_rate.0 as f64 * frame_rate.1 as f64);

        // ساخت یک مبدل برای تبدیل فرمت فریم‌ها به RGBA که برای canvas مناسب است
        let mut scaler = ffmpeg::software::scaling::Context::get(
          decoder.format(),
          decoder.width(),
          decoder.height(),
          ffmpeg::format::Pixel::RGBA, // فرمت مقصد
          decoder.width(),
          decoder.height(),
          ffmpeg::software::scaling::Flags::BILINEAR,
        )
        .unwrap();

        // حلقه اصلی برای خواندن بسته‌ها و دیکود کردن فریم‌ها
        for (stream, packet) in ictx.packets() {
          if stream.index() == video_stream_index {
            decoder.send_packet(&packet).unwrap();
            let mut decoded_frame = ffmpeg::util::frame::video::Video::empty();
            while decoder.receive_frame(&mut decoded_frame).is_ok() {
              let mut rgba_frame = ffmpeg::util::frame::video::Video::empty();
              scaler.run(&decoded_frame, &mut rgba_frame).unwrap();

              // ارسال داده‌های فریم به فرانت‌اند از طریق یک ایونت
              // ما داده‌های خام تصویر (slice) را می‌فرستیم
              window.emit("new_frame", rgba_frame.data(0).to_vec()).unwrap();
              
              // انتظار به اندازه فاصله بین فریم‌ها برای حفظ سرعت پخش اصلی
              thread::sleep(frame_interval);
            }
          }
        }
      }
      Err(err) => {
        // ارسال خطا به فرانت‌اند
        window.emit("playback_error", format!("Error opening file: {}", err)).unwrap();
      }
    }
  });
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_playback]) // رجیستر کردن کامند
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}