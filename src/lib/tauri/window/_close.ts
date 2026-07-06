import { invoke } from '@tauri-apps/api/core';

async function close() {
  return invoke('exit');
}

export default close;
