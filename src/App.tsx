// In src/App.tsx

import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';

function App() {
  const [status, setStatus] = useState<string>('');
  const [isLoading, setIsLoading] = useState<boolean>(false);

  const handleInstall = async () => {
    setIsLoading(true);
    setStatus('Installing FFmpeg, please wait...');
    try {
      const response = await invoke<string>('install_ffmpeg');
      setStatus(response);
    } catch (error) {
      setStatus(`Installation failed: ${error}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="App">
      <h1>FFmpeg Installer</h1>
      <button onClick={handleInstall} disabled={isLoading}>
        {isLoading ? 'Installing...' : 'Install FFmpeg'}
      </button>
      {status && <p>{status}</p>}
    </div>
  );
}

export default App;