import { invoke } from '@tauri-apps/api/core';

async function reload() {
  return invoke('reload');
}

export default reload;
