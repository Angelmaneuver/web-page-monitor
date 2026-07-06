import { invoke } from '@tauri-apps/api/core';

async function reload() {
  return invoke<void>('reload');
}

export default reload;
