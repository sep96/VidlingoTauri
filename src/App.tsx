// src/App.tsx
import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/api/dialog';
import './App.css';

interface VideoInfo {
  width: number;
  height: number;
}

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [videoPath, setVideoPath] = useState<string | null>(null);

  useEffect(() => {
    let unlistenInfo: (() => void) | undefined;
    let unlistenFrame: (() => void) | undefined;

    async function setupListeners() {
      // شنونده برای دریافت اطلاعات اولیه ویدیو (ابعاد)
      unlistenInfo = await listen<VideoInfo>('video_info', (event) => {
        const { width, height } = event.payload;
        const canvas = canvasRef.current;
        if (canvas) {
          canvas.width = width;
          canvas.height = height;
        }
      });

      // شنونده برای دریافت فریم‌های جدید
      unlistenFrame = await listen<number[]>('new_frame', (event) => {
        const canvas = canvasRef.current;
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        if (!ctx) return;

        // داده‌های فریم را به فرمت مناسب برای canvas تبدیل و نمایش می‌دهیم
        const frameData = new Uint8ClampedArray(event.payload);
        const imageData = new ImageData(frameData, canvas.width, canvas.height);
        ctx.putImageData(imageData, 0, 0);
      });
    }

    setupListeners();

    // پاک‌سازی شنونده‌ها زمانی که کامپوننت از بین می‌رود
    return () => {
      unlistenInfo?.();
      unlistenFrame?.();
    };
  }, []);

  const handleOpenFile = async () => {
    const path = await open({
      multiple: false,
      filters: [{ name: 'Video', extensions: ['mp4', 'mkv', 'avi', 'mov'] }],
    });

    if (typeof path === 'string') {
      setVideoPath(path);
      // فراخوانی کامند بک‌اند برای شروع پردازش ویدیو
      await invoke('start_playback', { videoPath: path });
    }
  };

  return (
    <div className="container">
      <h1>Tauri + React + FFmpeg Video Player 🎬</h1>
      <canvas ref={canvasRef} id="video-canvas"></canvas>
      <button onClick={handleOpenFile}>Open and Play Video</button>
      {videoPath && <p>Now Playing: {videoPath}</p>}
    </div>
  );
}

export default App;