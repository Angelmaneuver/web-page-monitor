import { invoke } from '@tauri-apps/api/core';

async function getAppLabel() {
  return invoke<string>('get_app_label');
}

export default getAppLabel;
