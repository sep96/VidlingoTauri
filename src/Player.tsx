// In your new video player project's src/Player.tsx

import React, { useState } from 'react';
import { open } from '@tauri-apps/api/dialog';
import { convertFileSrc } from '@tauri-apps/api/tauri';

function VideoPlayer() {
  const [videoSrc, setVideoSrc] = useState<string | null>(null);

  const selectVideo = async () => {
    const selectedPath = await open({
      multiple: false,
      filters: [{ name: 'Video', extensions: ['mp4', 'mkv', 'avi'] }]
    });

    if (typeof selectedPath === 'string') {
      // Convert the local file path to a URL the webview can use
      const assetUrl = convertFileSrc(selectedPath);
      setVideoSrc(assetUrl);
    }
  };

  return (
    <div>
      <button onClick={selectVideo}>Select Video</button>
      {videoSrc && (
        <video controls width="800" src={videoSrc}>
          Your browser does not support the video tag.
        </video>
      )}
    </div>
  );
}